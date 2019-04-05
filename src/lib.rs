#![allow(unused)]

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
