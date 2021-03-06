use crate::shared::Error;
use crate::shared::Result;

use crate::graphics_subsystem::device_context::ScopedDeviceContext;
use crate::graphics_subsystem::Point;
use crate::graphics_subsystem::RGBColor;
use crate::graphics_subsystem::Size;
use crate::graphics_subsystem::Rect;
use crate::graphics_subsystem::TenaryROP;
use crate::shared::booleanize;

pub trait Draw {
    fn draw<'a>(self, dc: &mut ScopedDeviceContext<'a>) -> Result<()>;
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn draw<D: Draw>(&mut self, v: D) -> Result<&mut Self> {
        v.draw(self)?;
        Ok(self)
    }

    pub fn draw_from_iter<I, D>(&mut self, iter: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = D>,
        D: Draw,
    {
        for v in iter {
            v.draw(self)?;
        }
        Ok(self)
    }
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn move_to(&mut self, pos: Point) -> Result<&mut Self> {
        use std::ptr::null_mut;
        use winapi::um::wingdi::MoveToEx;
        unsafe {
            if !booleanize(MoveToEx(
                self.data_ref().raw_handle(),
                pos.0.x,
                pos.0.y,
                null_mut(),
            )) {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn swap_to(&mut self, pos: &mut Point) -> Result<&mut Self> {
        use winapi::um::wingdi::MoveToEx;
        unsafe {
            if !booleanize(MoveToEx(
                self.data_ref().raw_handle(),
                pos.0.x,
                pos.0.y,
                &mut pos.0,
            )) {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn line_to(&mut self, pos: Point) -> Result<&mut Self> {
        use winapi::um::wingdi::LineTo;
        unsafe {
            if !booleanize(LineTo(self.data_ref().raw_handle(), pos.0.x, pos.0.y)) {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn get_pixel(&mut self, pos: Point) -> Result<Option<RGBColor>> {
        use winapi::um::wingdi::GetPixel;
        use winapi::um::wingdi::CLR_INVALID;
        let color = unsafe {
            let r = GetPixel(self.data_ref().raw_handle(), pos.0.x, pos.0.y);
            r
        };
        if color == CLR_INVALID {
            Ok(None)
        } else {
            Ok(Some(RGBColor(color)))
        }
    }

    pub fn set_background_color(&mut self, color: RGBColor) -> Result<&mut Self> {
        use winapi::um::wingdi::SetBkColor;
        use winapi::um::wingdi::CLR_INVALID;
        unsafe {
            let r = SetBkColor(self.data_ref().raw_handle(), color.into());
            if r == CLR_INVALID {
                return Error::last();
            }
        };
        Ok(self)
    }

    pub fn swap_background_color(&mut self, color: &mut RGBColor) -> Result<&mut Self> {
        use winapi::um::wingdi::SetBkColor;
        use winapi::um::wingdi::CLR_INVALID;
        let old_color = unsafe {
            let r = SetBkColor(self.data_ref().raw_handle(), (*color).into());
            if r == CLR_INVALID {
                return Error::last();
            }
            RGBColor(r)
        };
        *color = old_color;
        Ok(self)
    }

    pub fn fill_rect_with_background_color(&mut self, rect: Rect) -> Result<&mut Self> {
        use winapi::um::wingdi::ExtTextOutW;
        use winapi::um::wingdi::ETO_OPAQUE;
        use winapi::shared::windef::RECT;
        use std::ptr::null_mut;
        let rect: RECT = rect.into();
        unsafe {
            let r = ExtTextOutW(self.data_ref().raw_handle(), 0, 0, ETO_OPAQUE, 
                &rect as _, null_mut(),  0,  null_mut());
            if !booleanize(r) {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn bitblt(
        &mut self,
        src_dc: &ScopedDeviceContext,
        src_pos: Point,
        dest_pos: Point,
        size: Size,
        rop: TenaryROP,
    ) -> Result<&mut Self> {
        use winapi::um::wingdi::BitBlt;
        unsafe {
            if !booleanize(BitBlt(
                self.data_ref().raw_handle(),
                dest_pos.0.x,
                dest_pos.0.y,
                size.0.cx,
                size.0.cy,
                src_dc.data_ref().raw_handle(),
                src_pos.0.x,
                src_pos.0.y,
                rop.into(),
            )) {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn transparentblt(
        &mut self,
        src_dc: &ScopedDeviceContext,
        src_pos: Point,
        src_size: Size,
        dest_pos: Point,
        dest_size: Size,
        key: RGBColor,
    ) -> Result<&mut Self> {
        use winapi::um::wingdi::TransparentBlt;
        unsafe {
            if !booleanize(TransparentBlt(
                self.data_ref().raw_handle(),
                dest_pos.0.x,
                dest_pos.0.y,
                dest_size.0.cx,
                dest_size.0.cy,
                src_dc.data_ref().raw_handle(),
                src_pos.0.x,
                src_pos.0.y,
                src_size.0.cx,
                src_size.0.cy,
                key.into(),
            )) {
                return Error::last();
            }
        }
        Ok(self)
    }
}
