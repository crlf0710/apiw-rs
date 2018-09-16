use winapi::um::winuser::PAINTSTRUCT;
use winapi::shared::windef::HWND;
use winapi::shared::windef::HDC;
use wio::Result;
use wio::error::last_error;

use windows_subsystem::window::Window;
use graphics_subsystem::device_context::DeviceContext;
use utils::Temporary;
use utils::Handle;

pub struct PaintDeviceContext {
    window: HWND,
    paint_structure: PAINTSTRUCT,
    device_context: DeviceContext,
}

impl PaintDeviceContext {
    pub fn raw_handle(&self) -> HDC {
        self.device_context.raw_handle()
    }
}

impl Handle for PaintDeviceContext {
    fn clean_up(&mut self) {
        use winapi::um::winuser::EndPaint;
        unsafe {
            let _retvalue = EndPaint(self.window, &mut self.paint_structure);
        }
    }
}

impl Window {
    pub fn do_paint(&self) -> Result<Temporary<PaintDeviceContext>> {
        use winapi::um::winuser::BeginPaint;
        use std::mem::zeroed;
        use std::ptr::null_mut;
        let paint_dc = unsafe {
            let hwnd = self.raw_handle();
            let mut paint_structure = zeroed();
            let hdc = BeginPaint(hwnd, &mut paint_structure);
            if hdc.is_null() {
                return last_error();
            };
            PaintDeviceContext {
                window: hwnd,
                paint_structure: paint_structure,
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