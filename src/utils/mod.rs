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
    fn duplicate(&self) -> Self;
    fn clean_up(&mut self);
}

use std::marker::PhantomData;

pub struct Managed<'a, T: Handle>(T, PhantomData<&'a T>);

impl<'a, T: Handle> Managed<'a, T> {
    pub fn attach(v: T) -> Self {
        Managed(v, PhantomData)
    }

    pub fn share<'b, 'c>(&'b self) -> Managed<'c, T>
    where
        'a: 'c,
        'b: 'c,
    {
        Managed(self.0.duplicate(), PhantomData)
    }
}

use std::convert::AsRef;
use std::ops::Deref;

impl<'a, T: Handle> AsRef<T> for Managed<'a, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<'a, T: Handle> Deref for Managed<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct Temporary<'a, T: Handle>(T, PhantomData<&'a T>);

impl<'a, T: Handle> Temporary<'a, T> {
    pub fn attach(v: T) -> Self {
        Temporary(v, PhantomData)
    }

    pub fn share<'b, 'c>(&'b self) -> Managed<'c, T>
    where
        'a: 'c,
        'b: 'c,
    {
        Managed(self.0.duplicate(), PhantomData)
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

#[derive(Clone, Default)]
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

    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

impl<T: ToWide> From<T> for CWideString {
    fn from(v: T) -> Self {
        CWideString(v.to_wide_null())
    }
}
