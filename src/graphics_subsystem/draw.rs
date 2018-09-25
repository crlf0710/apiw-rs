use wio::Result;
use wio::error::last_error;

use utils::booleanize;
use graphics_subsystem::Point;
use graphics_subsystem::Size;
use graphics_subsystem::TenaryROP;
use graphics_subsystem::device_context::ScopedDeviceContext;
use graphics_subsystem::RGBColor;

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
        use winapi::um::wingdi::MoveToEx;
        use std::ptr::null_mut;
        unsafe
        {
            if !booleanize(MoveToEx(self.data_ref().raw_handle(), pos.0.x, pos.0.y, null_mut())) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn swap_to(&mut self, pos: &mut Point) -> Result<&mut Self> {
        use winapi::um::wingdi::MoveToEx;
        unsafe
        {
            if !booleanize(MoveToEx(self.data_ref().raw_handle(), pos.0.x, pos.0.y, &mut pos.0)) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn line_to(&mut self, pos: Point) -> Result<&mut Self> {
        use winapi::um::wingdi::LineTo;
        unsafe
        {
            if !booleanize(LineTo(self.data_ref().raw_handle(), pos.0.x, pos.0.y)) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn bitblt(&mut self, src_dc: &ScopedDeviceContext, src_pos: Point, dest_pos: Point, size: Size, rop: TenaryROP)
        -> Result<&mut Self>
    {
        use winapi::um::wingdi::BitBlt;
        unsafe {
            if !booleanize(BitBlt(self.data_ref().raw_handle(), dest_pos.0.x, dest_pos.0.y,
                size.0.cx, size.0.cy, src_dc.data_ref().raw_handle(),  src_pos.0.x, src_pos.0.y,
                rop.into())) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn transparentblt(&mut self, src_dc: &ScopedDeviceContext, src_pos: Point, src_size: Size,
                          dest_pos: Point, dest_size: Size, key: RGBColor)
                  -> Result<&mut Self>
    {
        use winapi::um::wingdi::TransparentBlt;
        unsafe {
            if !booleanize(TransparentBlt(self.data_ref().raw_handle(),
                                          dest_pos.0.x, dest_pos.0.y,
                                          dest_size.0.cx, dest_size.0.cy,
                                  src_dc.data_ref().raw_handle(),
                                          src_pos.0.x, src_pos.0.y,
                                          src_size.0.cx, src_size.0.cy,
                                          key.into())) {
                return last_error();
            }
        }
        Ok(self)
    }
}