//! XInput 1.3/1.4 hooking for blocking Xbox controller inputs

use std::sync::atomic::{AtomicBool, Ordering};

use asdf_overlay_hook::DetourHook;
use once_cell::sync::OnceCell;
use tracing::{debug, trace};

use crate::backend::Backends;

// XInput structures
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct XinputState {
    dw_packet_number: u32,
    gamepad: XinputGamepad,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct XinputGamepad {
    w_buttons: u16,
    b_left_trigger: u8,
    b_right_trigger: u8,
    s_thumb_lx: i16,
    s_thumb_ly: i16,
    s_thumb_rx: i16,
    s_thumb_ry: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct XinputVibration {
    w_left_motor_speed: u16,
    w_right_motor_speed: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct XinputCapabilities {
    r#type: u8,
    sub_type: u8,
    flags: u16,
    gamepad: XinputGamepad,
    vibration: XinputVibration,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct XinputKeystroke {
    virtual_key: u16,
    unicode: u16,
    flags: u16,
    user_index: u8,
    hid_code: u8,
}

static HOOK: OnceCell<XInputHook> = OnceCell::new();
static BLOCKING_ENABLED: AtomicBool = AtomicBool::new(false);

struct XInputHook {
    get_state: DetourHook<XInputGetStateFn>,
    set_state: DetourHook<XInputSetStateFn>,
    get_capabilities: DetourHook<XInputGetCapabilitiesFn>,
    get_keystroke: Option<DetourHook<XInputGetKeystrokeFn>>,
}

type XInputGetStateFn = unsafe extern "system" fn(u32, *mut XinputState) -> u32;
type XInputSetStateFn = unsafe extern "system" fn(u32, *const XinputVibration) -> u32;
type XInputGetCapabilitiesFn = unsafe extern "system" fn(u32, u32, *mut XinputCapabilities) -> u32;
type XInputGetKeystrokeFn = unsafe extern "system" fn(u32, u32, *mut XinputKeystroke) -> u32;

/// Initialize XInput hooks
pub fn hook() -> anyhow::Result<()> {
    // Try XInput 1.4 first, then fall back to 1.3
    let xinput_modules = [
        windows::core::s!("xinput1_4.dll"),
        windows::core::s!("xinput1_3.dll"),
        windows::core::s!("xinput9_1_0.dll"),
    ];

    let mut module = None;
    for module_name in &xinput_modules {
        if let Ok(m) = unsafe {
            windows::Win32::System::LibraryLoader::LoadLibraryA(*module_name)
        } {
            module = Some(m);
            debug!("Loaded XInput module: {:?}", module_name);
            break;
        }
    }

    let module = match module {
        Some(m) => m,
        None => {
            debug!("No XInput module found, skipping XInput hooks");
            return Ok(());
        }
    };

    // Get function pointers
    let get_state_proc = unsafe {
        windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows::core::s!("XInputGetState"),
        )
    };

    let set_state_proc = unsafe {
        windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows::core::s!("XInputSetState"),
        )
    };

    let get_capabilities_proc = unsafe {
        windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows::core::s!("XInputGetCapabilities"),
        )
    };

    let get_keystroke_proc = unsafe {
        windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows::core::s!("XInputGetKeystroke"),
        )
    };

    if get_state_proc.is_none() {
        debug!("XInputGetState not found");
        return Ok(());
    }

    // Hook functions
    debug!("Hooking XInputGetState");
    let get_state = unsafe {
        DetourHook::attach(
            std::mem::transmute(get_state_proc.unwrap()),
            hooked_xinput_get_state as _,
        )?
    };

    debug!("Hooking XInputSetState");
    let set_state = unsafe {
        DetourHook::attach(
            std::mem::transmute(set_state_proc.unwrap()),
            hooked_xinput_set_state as _,
        )?
    };

    debug!("Hooking XInputGetCapabilities");
    let get_capabilities = unsafe {
        DetourHook::attach(
            std::mem::transmute(get_capabilities_proc.unwrap()),
            hooked_xinput_get_capabilities as _,
        )?
    };

    let get_keystroke = if let Some(proc) = get_keystroke_proc {
        debug!("Hooking XInputGetKeystroke");
        Some(unsafe {
            DetourHook::attach(std::mem::transmute(proc), hooked_xinput_get_keystroke as _)?
        })
    } else {
        None
    };

    HOOK.set(XInputHook {
        get_state,
        set_state,
        get_capabilities,
        get_keystroke,
    })
    .map_err(|_| anyhow::anyhow!("XInput hook already initialized"))?;

    debug!("XInput hooks installed");
    Ok(())
}

/// Enable/disable XInput blocking
pub fn set_blocking(enabled: bool) {
    BLOCKING_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Check if XInput should be blocked
fn should_block_input() -> bool {
    if !BLOCKING_ENABLED.load(Ordering::Relaxed) {
        return false;
    }

    Backends::iter().any(|backend| backend.proc.lock().input_blocking())
}

const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
const ERROR_SUCCESS: u32 = 0;

#[tracing::instrument]
unsafe extern "system" fn hooked_xinput_get_state(
    dw_user_index: u32,
    p_state: *mut XinputState,
) -> u32 {
    trace!("XInputGetState called for controller {}", dw_user_index);

    if should_block_input() {
        // Return empty/neutral state when blocked
        if !p_state.is_null() {
            unsafe {
                p_state.write(XinputState {
                    dw_packet_number: 0,
                    gamepad: XinputGamepad {
                        w_buttons: 0,
                        b_left_trigger: 0,
                        b_right_trigger: 0,
                        s_thumb_lx: 0,
                        s_thumb_ly: 0,
                        s_thumb_rx: 0,
                        s_thumb_ry: 0,
                    },
                });
            }
        }
        return ERROR_SUCCESS;
    }

    unsafe { HOOK.wait().get_state.original_fn()(dw_user_index, p_state) }
}

#[tracing::instrument]
unsafe extern "system" fn hooked_xinput_set_state(
    dw_user_index: u32,
    p_vibration: *const XinputVibration,
) -> u32 {
    trace!("XInputSetState called for controller {}", dw_user_index);

    if should_block_input() {
        // Prevent vibration when blocked
        return ERROR_SUCCESS;
    }

    unsafe { HOOK.wait().set_state.original_fn()(dw_user_index, p_vibration) }
}

#[tracing::instrument]
unsafe extern "system" fn hooked_xinput_get_capabilities(
    dw_user_index: u32,
    dw_flags: u32,
    p_capabilities: *mut XinputCapabilities,
) -> u32 {
    trace!("XInputGetCapabilities called");

    // Always pass through - blocking shouldn't affect capability detection
    unsafe {
        HOOK.wait().get_capabilities.original_fn()(dw_user_index, dw_flags, p_capabilities)
    }
}

#[tracing::instrument]
unsafe extern "system" fn hooked_xinput_get_keystroke(
    dw_user_index: u32,
    dw_reserved: u32,
    p_keystroke: *mut XinputKeystroke,
) -> u32 {
    trace!("XInputGetKeystroke called");

    if should_block_input() {
        // No keystroke available when blocked
        return ERROR_DEVICE_NOT_CONNECTED;
    }

    unsafe {
        HOOK.wait()
            .get_keystroke
            .as_ref()
            .unwrap()
            .original_fn()(dw_user_index, dw_reserved, p_keystroke)
    }
}
