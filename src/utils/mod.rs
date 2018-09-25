use std::ptr::null_mut;
use winapi;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::HINSTANCE;
use wio::wide::{ToWide};
use wio::Result;

pub(crate) fn booleanize(v: BOOL) -> bool {
    v != 0
}
pub(crate) fn revert_booleanize(v: bool) -> BOOL {
    use winapi::shared::minwindef::{TRUE, FALSE};
    if v {
        TRUE
    } else {
        FALSE
    }
}

#[inline]
pub(crate) fn clamp_usize_to_positive_isize(v: usize) -> isize {
    use std::isize::{MAX};
    if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

#[inline]
pub(crate) fn clamp_isize_to_i32(v: isize) -> i32 {
    use std::i32::{MAX, MIN};
    if v < MIN as _ {
        MIN
    } else if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

#[inline]
pub(crate) fn clamp_i32_to_positive_i32(v: i32) -> i32 {
    if v < 0 {
        0
    } else {
        v
    }
}

#[inline]
pub(crate) fn clamp_usize_to_positive_i32(v: usize) -> i32 {
    use std::i32::{MAX};
    if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}

#[inline]
pub(crate) fn clamp_isize_to_positive_i32(v: isize) -> i32 {
    use std::i32::{MAX};
    if v < 0 {
        0
    } else if v > MAX as _ {
        MAX
    } else {
        v as _
    }
}


pub fn exe_cmd_show() -> winapi::ctypes::c_int {
    return winapi::um::winuser::SW_SHOW;
    // FIXME: This should be retrieved from GetStartupInfo().
    unimplemented!();
}

pub fn exe_instance() -> HINSTANCE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleW(null_mut()) }
}

pub trait OkOrLastError<T> {
    fn ok_or_last_error(self) -> Result<T>;
}

impl<T> OkOrLastError<T> for Option<T> {
    fn ok_or_last_error(self) -> Result<T> {
        use wio::error::last_error;
        if let Some(v) = self {
            Ok(v)
        } else {
            last_error()
        }
    }
}

impl<T> OkOrLastError<* mut T> for * mut T {
    fn ok_or_last_error(self) -> Result<* mut T> {
        use wio::error::last_error;
        if !self.is_null() {
            Ok(self)
        } else {
            last_error()
        }
    }
}

impl<T> OkOrLastError<* const T> for * const T {
    fn ok_or_last_error(self) -> Result<* const T> {
        use wio::error::last_error;
        if !self.is_null() {
            Ok(self)
        } else {
            last_error()
        }
    }
}



pub trait ManagedStrategy {
    fn clean_up<D: ManagedData>(&mut self, data: &mut D);
}

pub trait ManagedData {
    fn share(&self) -> Self;
    fn delete(&mut self);
}


pub struct ManagedEntity<D: ManagedData, T: ManagedStrategy> {
    data: D,
    strategy: T,
}

impl<D: ManagedData, T: ManagedStrategy> ManagedEntity<D, T> {
    pub(crate) fn data_ref(&self) -> &D {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}


impl<D: ManagedData, T: ManagedStrategy> Drop for ManagedEntity<D, T> {
    fn drop(&mut self) {
        self.strategy.clean_up(&mut self.data)
    }
}

impl<'a, D: ManagedData + 'a> Clone for ManagedEntity<D, strategy::LocalRc<'a>> {
    fn clone(&self) -> Self {
        let rc = self.strategy.clone();
        rc.cloned_entity(&self.data)
    }
}


pub mod strategy {
    use std::rc::Rc;
    use std::marker::PhantomData;
    use utils::ManagedStrategy;
    use utils::ManagedData;
    use utils::ManagedEntity;

    pub struct Foreign;

    impl Foreign {
        pub fn attached_entity<D: ManagedData>(data: D) -> ManagedEntity<D, Self> {
            ManagedEntity {
                data,
                strategy: Foreign,
            }
        }
    }


    impl ManagedStrategy for Foreign {
        fn clean_up<D: ManagedData>(&mut self, _data: &mut D) {
            // since we don't own this data, we do nothing.
        }
    }

    pub struct Local<'a>(PhantomData<&'a ()>);

    impl<'a> Local<'a> {
        pub fn attached_entity<D: ManagedData + 'a>(data: D) -> ManagedEntity<D, Self> {
            ManagedEntity {
                data,
                strategy: Local(PhantomData),
            }
        }
    }

    impl<'a> ManagedStrategy for Local<'a> {
        fn clean_up<D: ManagedData>(&mut self, data: &mut D) {
            // we own this data, so we ask the data to self destruct.
            data.delete()
        }
    }

    #[derive(Clone)]
    pub struct LocalRc<'a>(Option<Rc<()>>, PhantomData<&'a ()>);

    impl<'a> LocalRc<'a> {
        pub fn attached_entity<D: ManagedData + 'a>(data: D) -> ManagedEntity<D, Self> {
            ManagedEntity {
                data,
                strategy: LocalRc(Some(Rc::new(())), PhantomData),
            }
        }

        pub fn cloned_entity<D: ManagedData + 'a>(self, data: &D) -> ManagedEntity<D, Self> {
            ManagedEntity {
                data: data.share(),
                strategy: self,
            }
        }
    }

    impl<'a> ManagedStrategy for LocalRc<'a> {
        fn clean_up<D: ManagedData>(&mut self, data: &mut D) {
            // we share this data, so we ask the data to self destruct
            // if it is the last instance.
            if let Some(counter) = self.0.take() {
                if Rc::strong_count(&counter) == 1 {
                    data.delete()
                }
            }
        }
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
