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

pub mod shared;

pub mod windows_subsystem;

pub mod graphics_subsystem;

pub mod application_support_functions;

pub mod extensions;

pub mod full_windows_api {
    #[doc(inline)]
    pub use winapi::*;
}

pub use crate::shared::{Result, last_error, maybe_last_error};
