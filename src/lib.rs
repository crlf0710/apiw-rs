#![allow(unused)]

extern crate winapi;
extern crate wio;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
#[macro_use]
extern crate log;

pub mod windows_subsystem;

pub mod graphics_subsystem;

pub mod application_support_functions;

pub mod utils;

pub mod full_windows_api {
    #[doc(inline)]
    pub use winapi::*;
}

pub use wio::Result;
pub use wio::error::last_error;

pub fn maybe_last_error<T, D: FnOnce() -> T>(f: D) -> wio::Result<T> {
    let err = last_error();
    let code = if let Err(ref err) = err {
        err.code()
    } else {
        0
    };
    if code == 0 {
        Ok(f())
    } else {
        err
    }
}