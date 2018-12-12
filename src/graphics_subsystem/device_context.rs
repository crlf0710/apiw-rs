use winapi::shared::windef::HDC;
use wio::error::last_error;
use wio::Result;

use shared::booleanize;
use shared::strategy;
use shared::ManagedData;
use shared::ManagedEntity;
use shared::ManagedStrategy;

use graphics_subsystem::object::{Bitmap, Pen};
use winapi::shared::windef::HBITMAP;
use winapi::shared::windef::HPEN;

pub type AnyDeviceContext<T> = ManagedEntity<DeviceContextInner, T>;
pub type ScopedDeviceContext<'a> = AnyDeviceContext<strategy::Local<'a>>;
pub type LocalDeviceContext = ScopedDeviceContext<'static>;

#[derive(Clone, Copy)]
pub(crate) enum DeviceContextInnerKind {
    Normal,
    Special,
}

pub struct DeviceContextInner {
    handle: HDC,
    kind: DeviceContextInnerKind,
    tracking_pen_original: Option<HPEN>,
    tracking_pen_active: Option<Pen>,
    tracking_bitmap_original: Option<HBITMAP>,
    tracking_bitmap_active: Option<Bitmap>,
}

impl DeviceContextInner {
    pub(crate) fn new_initial_dc_from_attached(dc: HDC, kind: DeviceContextInnerKind) -> Self {
        DeviceContextInner {
            handle: dc,
            kind,
            tracking_pen_original: None,
            tracking_pen_active: None,
            tracking_bitmap_original: None,
            tracking_bitmap_active: None,
        }
    }

    pub(crate) fn raw_handle(&self) -> HDC {
        self.handle
    }

    pub(crate) fn track_old_pen(&mut self, v: HPEN) {
        if self.tracking_pen_original.is_none() {
            self.tracking_pen_original = Some(v);
        }
    }

    pub(crate) fn track_active_pen(&mut self, v: Pen) {
        self.tracking_pen_active = Some(v);
    }

    pub(crate) fn track_old_bitmap(&mut self, v: HBITMAP) {
        if self.tracking_bitmap_original.is_none() {
            self.tracking_bitmap_original = Some(v);
        }
    }

    pub(crate) fn track_active_bitmap(&mut self, v: Bitmap) {
        self.tracking_bitmap_active = Some(v);
    }

    pub(crate) fn restore_to_tracked_state(&mut self) {
        use winapi::um::wingdi::SelectObject;

        if let Some(old_pen) = self.tracking_pen_original.take() {
            unsafe {
                let h = SelectObject(self.raw_handle(), old_pen as _);
                if h.is_null() {
                    warn!(target: "apiw", "Failed to restore {} state for {}, last error: {:?}",
                          "pen", "DeviceContext", last_error::<()>());
                }
            }
        }
        self.tracking_pen_active = None;

        if let Some(old_bitmap) = self.tracking_bitmap_original.take() {
            unsafe {
                let h = SelectObject(self.raw_handle(), old_bitmap as _);
                if h.is_null() {
                    warn!(target: "apiw", "Failed to restore {} state for {}, last error: {:?}",
                          "bitmap", "DeviceContext", last_error::<()>());
                }
            }
        }
        self.tracking_bitmap_active = None;
    }
}

impl ManagedData for DeviceContextInner {
    fn share(&self) -> Self {
        panic!("DeviceContext cannot be shared.");
    }

    fn delete(&mut self) {
        use winapi::um::wingdi::DeleteDC;

        self.restore_to_tracked_state();

        let kind = self.kind;
        match kind {
            DeviceContextInnerKind::Normal => unsafe {
                let succeeded = booleanize(DeleteDC(self.raw_handle()));
                if !succeeded {
                    warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "LocalDeviceContext", last_error::<()>());
                }
            },
            DeviceContextInnerKind::Special => {
                // do nothing here.
            }
        }
    }
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn reset_to_initial_state(&mut self) {
        self.data_mut().restore_to_tracked_state();
    }
}

impl LocalDeviceContext {
    pub fn new_compatible_memory_dc(dc: &ScopedDeviceContext) -> Result<LocalDeviceContext> {
        use winapi::um::wingdi::CreateCompatibleDC;
        let memdc = unsafe {
            let h = CreateCompatibleDC(dc.data_ref().raw_handle());
            if h.is_null() {
                return last_error();
            }
            h
        };

        Ok(strategy::Local::attached_entity(
            DeviceContextInner::new_initial_dc_from_attached(memdc, DeviceContextInnerKind::Normal),
        ))
    }

    pub fn new_compatible_memory_dc_for_current_screen() -> Result<LocalDeviceContext> {
        use std::ptr::null_mut;
        use winapi::um::wingdi::CreateCompatibleDC;
        let memdc = unsafe {
            let h = CreateCompatibleDC(null_mut());
            if h.is_null() {
                return last_error();
            }
            h
        };

        Ok(strategy::Local::attached_entity(
            DeviceContextInner::new_initial_dc_from_attached(memdc, DeviceContextInnerKind::Normal),
        ))
    }
}
