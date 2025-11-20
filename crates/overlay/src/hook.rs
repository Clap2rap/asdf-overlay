//! Collection of hooks required to intercept window events and rendering.

mod dx;
mod opengl;
mod proc;
mod dinput;
mod xinput;

pub mod util {
    pub use super::dx::original_execute_command_lists;
}

use anyhow::Context;
use windows::Win32::Foundation::HINSTANCE;

use crate::util::with_dummy_hwnd;

#[tracing::instrument]
/// Install various hooks.
pub fn install(hinstance: HINSTANCE) -> anyhow::Result<()> {
    with_dummy_hwnd(hinstance, |dummy_hwnd| {
        proc::hook().context("Proc hook failed")?;
        dx::hook(dummy_hwnd);
        opengl::hook(dummy_hwnd);

        // Add DirectInput and XInput hooks
        if let Err(e) = dinput::hook(dummy_hwnd) {
            tracing::warn!("DirectInput hook failed: {:?}", e);
        }

        if let Err(e) = xinput::hook() {
            tracing::warn!("XInput hook failed: {:?}", e);
        }

        Ok(())
    })?
}

/// Enable/disable DirectInput blocking
pub use dinput::set_blocking as set_dinput_blocking;

/// Enable/disable XInput blocking
pub use xinput::set_blocking as set_xinput_blocking;
