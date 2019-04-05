use wio::error::last_error;
use wio::Result;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::FLOAT;
use crate::shared::booleanize;
use crate::graphics_subsystem::device_context::ScopedDeviceContext;

pub struct Transform(winapi::um::wingdi::XFORM);

impl Transform {
    const fn new() -> Self {
        Self::new_with_values(&[1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }
    const fn new_with_values(values: &[FLOAT; 6]) -> Self {
        use winapi::um::wingdi::XFORM;
        Transform(XFORM {
            eM11: values[0],
            eM12: values[1],
            eM21: values[2],
            eM22: values[3],
            eDx: values[4],
            eDy: values[5],
        })
    }
}


#[derive(PartialEq)]
pub struct GraphicsMode(c_int);

impl GraphicsMode {
    const COMPATIBLE: GraphicsMode = GraphicsMode(winapi::um::wingdi::GM_COMPATIBLE as c_int);
    const ADVANCED: GraphicsMode = GraphicsMode(winapi::um::wingdi::GM_ADVANCED as c_int);
}

impl<'a> ScopedDeviceContext<'a> {
    pub fn set_graphics_mode(&mut self, graphics_mode: GraphicsMode) -> Result<&mut Self> {
        use winapi::um::wingdi::SetGraphicsMode;
        unsafe {
            let v = SetGraphicsMode(self.data_ref().raw_handle(), graphics_mode.0);
            if v == 0 {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn swap_graphics_mode(&mut self, graphics_mode: &mut GraphicsMode) -> Result<&mut Self> {
        use winapi::um::wingdi::SetGraphicsMode;
        unsafe {
            let v = SetGraphicsMode(self.data_ref().raw_handle(), graphics_mode.0);
            if v == 0 {
                return last_error();
            }
            graphics_mode.0 = v;
        }
        Ok(self)
    }

    pub fn reset_world_transform(&mut self) -> Result<&mut Self> {
        use winapi::um::wingdi::ModifyWorldTransform;
        use winapi::um::wingdi::MWT_IDENTITY;
        use std::ptr::null;
        unsafe {
            let v = ModifyWorldTransform(self.data_ref().raw_handle(), null(), MWT_IDENTITY as _);
            if !booleanize(v) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn set_world_transform(&mut self, transform: &Transform) -> Result<&mut Self> {
        use winapi::um::wingdi::SetWorldTransform;
        unsafe {
            let v = SetWorldTransform(self.data_ref().raw_handle(), &transform.0 as _);
            if !booleanize(v) {
                return last_error();
            }
        }
        Ok(self)
    }

    pub fn modify_world_transform_left_multiply(&mut self, transform: &Transform) -> Result<&mut Self> {
        use winapi::um::wingdi::ModifyWorldTransform;
        use winapi::um::wingdi::MWT_LEFTMULTIPLY;
        use std::ptr::null;
        unsafe {
            let v = ModifyWorldTransform(self.data_ref().raw_handle(), &transform.0 as _, MWT_LEFTMULTIPLY as _);
            if !booleanize(v) {
                return last_error();
            }
        }
        Ok(self)
    }
    
    pub fn modify_world_transform_right_multiply(&mut self, transform: &Transform) -> Result<&mut Self> {
        use winapi::um::wingdi::ModifyWorldTransform;
        use winapi::um::wingdi::MWT_RIGHTMULTIPLY;
        use std::ptr::null;
        unsafe {
            let v = ModifyWorldTransform(self.data_ref().raw_handle(), &transform.0 as _, MWT_RIGHTMULTIPLY as _);
            if !booleanize(v) {
                return last_error();
            }
        }
        Ok(self)
    }
}
