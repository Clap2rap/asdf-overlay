//! DirectInput 8 hooking for blocking gamepad/joystick inputs

use core::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use asdf_overlay_hook::DetourHook;
use once_cell::sync::OnceCell;
use tracing::{debug, trace};
use windows::{
    core::{GUID, HRESULT},
    Win32::Foundation::HWND,
};

use crate::backend::Backends;

// DirectInput8 COM interface vtable offsets
const DINPUT8_CREATEDEVICE_VTABLE_OFFSET: usize = 3;

static HOOK: OnceCell<DInputHook> = OnceCell::new();
static BLOCKING_ENABLED: AtomicBool = AtomicBool::new(false);

struct DInputHook {
    create_device: DetourHook<DirectInput8CreateDeviceFn>,
}

type DirectInput8CreateDeviceFn = unsafe extern "system" fn(
    *mut c_void,
    *const GUID,
    *mut *mut c_void,
    *mut c_void,
) -> HRESULT;

/// Initialize DirectInput hooks
pub fn hook(_hwnd: HWND) -> anyhow::Result<()> {
    // Load DirectInput8 library
    let dinput8_module = unsafe {
        windows::Win32::System::LibraryLoader::LoadLibraryW(windows::core::w!("dinput8.dll"))
    };

    let dinput8_module = match dinput8_module {
        Ok(m) => m,
        Err(_) => {
            debug!("dinput8.dll not found, skipping DirectInput hooks");
            return Ok(());
        }
    };

    // Get DirectInput8Create function
    let dinput8_create = unsafe {
        windows::Win32::System::LibraryLoader::GetProcAddress(
            dinput8_module,
            windows::core::s!("DirectInput8Create"),
        )
    };

    if dinput8_create.is_none() {
        debug!("DirectInput8Create not found, skipping DirectInput hooks");
        return Ok(());
    }

    // Create a temporary DirectInput instance to get vtable
    let mut dinput8: *mut c_void = std::ptr::null_mut();
    let hresult = unsafe {
        std::mem::transmute::<
            _,
            unsafe extern "system" fn(
                windows::Win32::Foundation::HINSTANCE,
                u32,
                *const GUID,
                *mut *mut c_void,
                *mut c_void,
            ) -> HRESULT,
        >(dinput8_create.unwrap())(
            windows::Win32::Foundation::HINSTANCE(dinput8_module.0 as _),
            0x0800, // DIRECTINPUT_VERSION
            &windows::core::GUID::from_u128(0x25E609E4_B16A_11CE_9E19_00AA0040401B), // IID_IDirectInput8W
            &mut dinput8 as *mut _,
            std::ptr::null_mut(),
        )
    };

    if hresult.is_err() || dinput8.is_null() {
        debug!("Failed to create DirectInput8 instance, error: {:?}", hresult);
        return Ok(());
    }

    // Hook CreateDevice from vtable
    let vtable = unsafe { *(dinput8 as *const *const *const c_void) };
    let create_device_fn = unsafe {
        std::mem::transmute::<*const c_void, DirectInput8CreateDeviceFn>(
            *vtable.add(DINPUT8_CREATEDEVICE_VTABLE_OFFSET),
        )
    };

    debug!("Hooking DirectInput8::CreateDevice");
    let create_device =
        unsafe { DetourHook::attach(create_device_fn, hooked_create_device as _)? };

    // Release temporary instance
    unsafe {
        let release_fn = std::mem::transmute::<*const c_void, unsafe extern "system" fn(*mut c_void) -> u32>(
            *vtable.add(2),
        );
        release_fn(dinput8);
    }

    HOOK.set(DInputHook { create_device })
        .map_err(|_| anyhow::anyhow!("DirectInput hook already initialized"))?;

    debug!("DirectInput hooks installed");
    Ok(())
}

/// Enable/disable DirectInput blocking
pub fn set_blocking(enabled: bool) {
    BLOCKING_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Check if DirectInput is currently blocked
fn should_block_input() -> bool {
    if !BLOCKING_ENABLED.load(Ordering::Relaxed) {
        return false;
    }

    // Check if any window has input blocking enabled
    Backends::iter().any(|backend| backend.proc.lock().input_blocking())
}

#[tracing::instrument]
unsafe extern "system" fn hooked_create_device(
    this: *mut c_void,
    rguid: *const GUID,
    lplpdevice: *mut *mut c_void,
    punkouter: *mut c_void,
) -> HRESULT {
    trace!("DirectInput8::CreateDevice called");

    let result =
        unsafe { HOOK.wait().create_device.original_fn()(this, rguid, lplpdevice, punkouter) };

    if result.is_ok() && !lplpdevice.is_null() {
        let device = unsafe { *lplpdevice };
        if !device.is_null() && should_block_input() {
            debug!("DirectInput device created while blocking is enabled");
            // Device methods would need additional hooking per-device
            // This is complex due to COM vtables - left for future enhancement
        }
    }

    result
}
