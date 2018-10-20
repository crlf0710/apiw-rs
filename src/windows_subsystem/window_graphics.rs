use winapi::shared::windef::HDC;
use winapi::shared::windef::HWND;
use winapi::um::winuser::PAINTSTRUCT;
use wio::error::last_error;
use wio::Result;

use utils::booleanize;
use utils::strategy;
use utils::{ManagedData, ManagedEntity};

use graphics_subsystem::device_context::LocalDeviceContext;
use graphics_subsystem::device_context::{DeviceContextInner, DeviceContextInnerKind};
use windows_subsystem::window::AnyWindow;

pub type AnyPaintDeviceContext<T> = ManagedEntity<PaintDeviceContextInner, T>;

pub type LocalPaintDeviceContext = AnyPaintDeviceContext<strategy::Local<'static>>;

pub struct PaintDeviceContextInner {
    window: HWND,
    paint_structure: Option<PAINTSTRUCT>,
    device_context: LocalDeviceContext,
}

impl PaintDeviceContextInner {
    pub(crate) fn raw_handle(&self) -> HDC {
        self.device_context.data_ref().raw_handle()
    }
}

use std::ops::Deref;
use std::ops::DerefMut;
use utils::ManagedStrategy;

impl<T: ManagedStrategy> Deref for AnyPaintDeviceContext<T> {
    type Target = LocalDeviceContext;
    fn deref(&self) -> &LocalDeviceContext {
        &self.data_ref().device_context
    }
}

impl<T: ManagedStrategy> DerefMut for AnyPaintDeviceContext<T> {
    fn deref_mut(&mut self) -> &mut LocalDeviceContext {
        &mut self.data_mut().device_context
    }
}

impl ManagedData for PaintDeviceContextInner {
    fn share(&self) -> Self {
        panic!("AnyPaintDeviceContext cannot be shared.");
        /*
        AnyPaintDeviceContext {
            window: self.window,
            paint_structure: None,
            device_context: self.device_context.duplicate(),
        }
        */
    }
    fn delete(&mut self) {
        use winapi::um::winuser::EndPaint;
        self.device_context.reset_to_initial_state();
        unsafe {
            if let Some(paint_structure) = self.paint_structure.as_mut() {
                let _retvalue = EndPaint(self.window, paint_structure);
            } else {
                warn!(target: "apiw", "No PAINTSTRUCT in TemporaryPaintDC.");
            }
        }
    }
}

impl<T: ManagedStrategy> AnyWindow<T> {
    pub fn do_paint(&self) -> Result<AnyPaintDeviceContext<strategy::Local>> {
        use std::mem::zeroed;
        use winapi::um::winuser::BeginPaint;
        let paint_dc: PaintDeviceContextInner = unsafe {
            let hwnd = self.data_ref().raw_handle();
            let mut paint_structure = zeroed();
            let hdc = BeginPaint(hwnd, &mut paint_structure);
            if hdc.is_null() {
                return last_error();
            };
            PaintDeviceContextInner {
                window: hwnd,
                paint_structure: Some(paint_structure),
                device_context: strategy::Local::attached_entity(
                    DeviceContextInner::new_initial_dc_from_attached(
                        hdc,
                        DeviceContextInnerKind::Special,
                    ),
                ),
            }
        };
        Ok(strategy::Local::attached_entity(paint_dc))
    }

    /// ECMA-234 Clause 156 UpdateWindow
    pub fn update(&self) -> Result<&Self> {
        use winapi::um::winuser::UpdateWindow;
        unsafe {
            UpdateWindow(self.data_ref().raw_handle());
        }
        Ok(self)
    }

    pub fn invalidate(&self) -> Result<&Self> {
        use std::ptr::null;
        use winapi::shared::minwindef::FALSE;
        use winapi::um::winuser::InvalidateRect;
        unsafe {
            if !booleanize(InvalidateRect(self.data_ref().raw_handle(), null(), FALSE)) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn invalidate_and_erase(&self) -> Result<&Self> {
        use std::ptr::null;
        use winapi::shared::minwindef::TRUE;
        use winapi::um::winuser::InvalidateRect;
        unsafe {
            if !booleanize(InvalidateRect(self.data_ref().raw_handle(), null(), TRUE)) {
                return last_error();
            }
        }
        Ok(self)
    }
}
