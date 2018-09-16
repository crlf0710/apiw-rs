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

pub trait Handle: 'static {
    fn clean_up(&mut self);
}

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

use std::marker::PhantomData;

pub struct Temporary<'a, T: Handle>(T, PhantomData<&'a T>);

impl<'a, T: Handle> Temporary<'a, T> {
    pub fn attach(v: T) -> Self {
        Temporary(v, PhantomData)
    }
}

impl<'a, T: Handle> Drop for Temporary<'a, T> {
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

pub struct CWideString(Vec<u16>);

impl CWideString {
    pub fn new() -> Self {
        CWideString(Vec::new())
    }

    pub fn is_null(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        if self.is_null() {
            0
        } else {
            self.len_with_null() - 1
        }
    }

    pub fn len_with_null(&self) -> usize {
        self.0.len()
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