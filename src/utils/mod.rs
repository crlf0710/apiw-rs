use std::ptr::null_mut;
use winapi;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::HINSTANCE;
use wio::wide::{FromWide, ToWide};

pub(crate) fn booleanize(v: BOOL) -> bool {
    v != 0
}

pub fn exe_cmd_show() -> winapi::ctypes::c_int {
    return winapi::um::winuser::SW_SHOW;
    // FIXME: This should be retrieved from GetStartupInfo().
    unimplemented!();
}

pub fn exe_instance() -> HINSTANCE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleW(null_mut()) }
}

pub trait Handle {}

pub struct Permanent<T: Handle>(T);

impl<T: Handle> Permanent<T> {
    pub fn attach(v: T) -> Self {
        Permanent(v)
    }
}

use std::convert::AsRef;
use std::ops::Deref;

impl<T: Handle> AsRef<T> for Permanent<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Handle> Deref for Permanent<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct Temporary<T: Handle>(T);

impl<T: Handle> AsRef<T> for Temporary<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

pub struct CWideString(Vec<u16>);

impl CWideString {
    pub fn new() -> Self {
        CWideString(Vec::new())
    }

    pub fn as_ptr(&self) -> * const u16 {
        self.0.as_ptr()
    }
}

impl<T: ToWide> From<T> for CWideString {
    fn from(v: T) -> Self {
        CWideString(v.to_wide_null())
    }
}