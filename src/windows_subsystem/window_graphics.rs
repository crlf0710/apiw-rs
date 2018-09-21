use winapi::shared::windef::HDC;
use winapi::shared::windef::HWND;
use winapi::um::winuser::PAINTSTRUCT;
use wio::error::last_error;
use wio::Result;

use graphics_subsystem::device_context::DeviceContext;
use utils::Handle;
use utils::Temporary;
use windows_subsystem::window::Window;

pub struct PaintDeviceContext {
    window: HWND,
    // None in Managed, Some in Temporary
    paint_structure: Option<PAINTSTRUCT>,
    device_context: DeviceContext,
}

impl PaintDeviceContext {
    pub fn raw_handle(&self) -> HDC {
        self.device_context.raw_handle()
    }
}

use std::ops::Deref;

impl Deref for PaintDeviceContext {
    type Target = DeviceContext;
    fn deref(&self) -> &DeviceContext {
        &self.device_context
    }
}

impl Handle for PaintDeviceContext {
    fn duplicate(&self) -> Self {
        PaintDeviceContext {
            window: self.window,
            paint_structure: None,
            device_context: self.device_context.duplicate(),
        }
    }
    fn clean_up(&mut self) {
        use winapi::um::winuser::EndPaint;
        unsafe {
            if let Some(paint_structure) = self.paint_structure.as_mut() {
                let _retvalue = EndPaint(self.window, paint_structure);
            } else {
                warn!(target: "apiw", "No PAINTSTRUCT in TemporaryPaintDC.");
            }
        }
    }
}

impl Window {
    pub fn do_paint(&self) -> Result<Temporary<PaintDeviceContext>> {
        use std::mem::zeroed;
        use std::ptr::null_mut;
        use winapi::um::winuser::BeginPaint;
        let paint_dc = unsafe {
            let hwnd = self.raw_handle();
            let mut paint_structure = zeroed();
            let hdc = BeginPaint(hwnd, &mut paint_structure);
            if hdc.is_null() {
                return last_error();
            };
            PaintDeviceContext {
                window: hwnd,
                paint_structure: Some(paint_structure),
                device_context: DeviceContext(hdc),
            }
        };
        Ok(Temporary::attach(paint_dc))
    }

    /// ECMA-234 Clause 156 UpdateWindow
    pub fn update(&self) -> Result<&Self> {
        use winapi::um::winuser::UpdateWindow;
        unsafe {
            UpdateWindow(self.raw_handle());
        }
        Ok(self)
    }
}
