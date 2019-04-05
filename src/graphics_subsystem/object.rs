use derive_more::Into;
use log::warn;
use std::cell::Cell;
use std::rc::Rc;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::HRGN;
use winapi::shared::minwindef::WORD;
use winapi::shared::windef::{HBITMAP, HBRUSH, HFONT, HPALETTE, HPEN};
use winapi::um::wingdi::{
    PS_DASH, PS_DASHDOT, PS_DASHDOTDOT, PS_DOT, PS_INSIDEFRAME, PS_NULL, PS_SOLID,
};
use wio::error::Error;
use wio::Result;

use crate::graphics_subsystem::device_context::ScopedDeviceContext;
use crate::graphics_subsystem::RGBColor;
use crate::shared;
use crate::shared::booleanize;
use crate::shared::clamp_usize_to_positive_i32;
use crate::shared::strategy;
use crate::shared::ManagedData;
use crate::shared::ManagedEntity;
use crate::shared::ManagedStrategy;

#[derive(Clone)]
pub struct PenInner(HPEN);

impl PenInner {
    pub fn raw_handle(&self) -> HPEN {
        self.0
    }
}

impl ManagedData for PenInner {
    fn share(&self) -> Self {
        self.clone()
    }

    fn delete(&mut self) {
        use winapi::um::wingdi::DeleteObject;
        unsafe {
            let succeeded = booleanize(DeleteObject(self.raw_handle() as _));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "Pen", Error::last::<()>());
            }
        }
    }
}

pub type Pen = ManagedEntity<PenInner, strategy::LocalRc<'static>>;

#[derive(Clone, Copy, Into)]
pub struct PenStyle(c_int);

impl PenStyle {
    pub const SOLID: PenStyle = PenStyle(PS_SOLID as _);
    pub const DASH: PenStyle = PenStyle(PS_DASH as _);
    pub const DOT: PenStyle = PenStyle(PS_DOT as _);
    pub const DASH_DOT: PenStyle = PenStyle(PS_DASHDOT as _);
    pub const DASH_DOT_DOT: PenStyle = PenStyle(PS_DASHDOTDOT as _);
    pub const NULL: PenStyle = PenStyle(PS_NULL as _);
    pub const INSIDE_FRAME: PenStyle = PenStyle(PS_INSIDEFRAME as _);
}

pub struct PenBuilder {
    style: PenStyle,
    width: usize,
    color: RGBColor,
}

impl PenBuilder {
    pub fn new() -> Self {
        PenBuilder {
            style: PenStyle::SOLID,
            width: 0,
            color: RGBColor::BLACK,
        }
    }

    pub fn style(mut self, style: PenStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: RGBColor) -> Self {
        self.color = color;
        self
    }

    pub fn create(self) -> Result<Pen> {
        use winapi::um::wingdi::CreatePen;
        let pen = unsafe {
            let h = CreatePen(
                self.style.into(),
                clamp_usize_to_positive_i32(self.width),
                self.color.into(),
            );
            if h.is_null() {
                return Error::last();
            }
            PenInner(h)
        };
        Ok(strategy::LocalRc::attached_entity(pen))
    }
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn select_pen(&mut self, pen: Pen) -> Result<&mut Self> {
        use winapi::um::wingdi::SelectObject;
        let old_pen = unsafe {
            let h = SelectObject(
                self.data_ref().raw_handle(),
                pen.data_ref().raw_handle() as _,
            );
            if h.is_null() {
                return Error::last();
            }
            self.data_mut().track_old_pen(h as _);
            self.data_mut().track_active_pen(pen);
        };
        Ok(self)
    }
}

pub struct BrushInner(HBRUSH);

pub type Brush = ManagedEntity<BrushInner, strategy::LocalRc<'static>>;

pub struct FontInner(HFONT);

pub type Font = ManagedEntity<FontInner, strategy::LocalRc<'static>>;

#[derive(Clone)]
pub struct BitmapInner(HBITMAP, Rc<Cell<bool>>);

impl BitmapInner {
    pub fn raw_handle(&self) -> HBITMAP {
        self.0
    }
}

impl ManagedData for BitmapInner {
    fn share(&self) -> Self {
        self.clone()
    }

    fn delete(&mut self) {
        use winapi::um::wingdi::DeleteObject;
        unsafe {
            let succeeded = booleanize(DeleteObject(self.raw_handle() as _));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "Bitmap", Error::last::<()>());
            }
        }
    }
}

pub type Bitmap = ManagedEntity<BitmapInner, strategy::LocalRc<'static>>;

impl Bitmap {
    pub fn load_from_resource_id(id: WORD) -> Result<Bitmap> {
        use crate::windows_subsystem::window::ResourceIDOrIDString;
        use winapi::um::winuser::LoadBitmapW;
        let resource = ResourceIDOrIDString::ID(id);
        let bitmap = unsafe {
            let h = LoadBitmapW(shared::exe_instance(), resource.as_ptr_or_int_ptr());
            if h.is_null() {
                return Error::last();
            }
            BitmapInner(h, Rc::new(Cell::new(false)))
        };
        Ok(strategy::LocalRc::attached_entity(bitmap))
    }
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn select_bitmap(&mut self, bitmap: Bitmap) -> Result<&mut Self> {
        use winapi::um::wingdi::SelectObject;
        let old_pen = unsafe {
            let h = SelectObject(
                self.data_ref().raw_handle(),
                bitmap.data_ref().raw_handle() as _,
            );
            if h.is_null() {
                return Error::last();
            }
            self.data_mut().track_old_bitmap(h as _);
            self.data_mut().track_active_bitmap(bitmap);
        };
        Ok(self)
    }
}

pub struct PaletteInner(HPALETTE); //FIXME

pub type Palette = ManagedEntity<PaletteInner, strategy::LocalRc<'static>>;

pub struct RegionInner(HRGN);

pub type Region = ManagedEntity<RegionInner, strategy::Local<'static>>;

/*

use std::rc::Rc;

pub struct TemporaryRc<'a, T: Handle>(Rc<T>, PhantomData<&'a Rc<T>>);

impl<'a, T: Handle> Drop for TemporaryRc<'a, T> {
    fn drop(&mut self) {
        <T as Handle>::clean_up(&mut self.0);
    }
}

impl<'a, T: Handle> AsRef<T> for Temporary<'a, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<'a, T: Handle> Deref for Temporary<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
*/
