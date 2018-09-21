use wio::Result;
use wio::error::last_error;

use utils::booleanize;
use graphics_subsystem::Point;
use graphics_subsystem::device_context::DeviceContext;

pub trait Draw {
    fn draw(self, dc: &DeviceContext) -> Result<()>;
}

impl DeviceContext {
    pub fn draw<D: Draw>(&self, v: D) -> Result<&Self> {
        v.draw(self)?;
        Ok(self)
    }

    pub fn draw_from_iter<T, D>(&self, iter: T) -> Result<&Self>
    where
        T: IntoIterator<Item = D>,
        D: Draw,
    {
        for v in iter {
            v.draw(self)?;
        }
        Ok(self)
    }
}

impl DeviceContext {
    pub fn move_to(&self, pos: Point) -> Result<&Self> {
        use winapi::um::wingdi::MoveToEx;
        use std::ptr::null_mut;
        unsafe
        {
            if !booleanize(MoveToEx(self.raw_handle(), pos.0.x, pos.0.y, null_mut())) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn swap_to(&self, pos: &mut Point) -> Result<&Self> {
        use winapi::um::wingdi::MoveToEx;
        unsafe
        {
            if !booleanize(MoveToEx(self.raw_handle(), pos.0.x, pos.0.y, &mut pos.0)) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn line_to(&self, pos: Point) -> Result<&Self> {
        use winapi::um::wingdi::LineTo;
        unsafe
        {
            if !booleanize(LineTo(self.raw_handle(), pos.0.x, pos.0.y)) {
                return last_error();
            }
        }
        Ok(self)
    }
}