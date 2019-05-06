#![allow(unused)]

#[cfg(windows)]
pub mod shared;

#[cfg(windows)]
pub mod windows_subsystem;

#[cfg(windows)]
pub mod graphics_subsystem;

#[cfg(windows)]
pub mod application_support_functions;

#[cfg(windows)]
pub mod extensions;

#[cfg(windows)]
pub mod full_windows_api {
    #[doc(inline)]
    pub use winapi::*;
}

#[cfg(windows)]
pub use crate::shared::{maybe_last_error, internal_error, Error, Result};
