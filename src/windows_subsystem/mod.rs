pub mod debug;
pub mod menu;
pub mod message;
pub mod window;
pub mod window_graphics;
pub mod dialog;

use winapi::shared::minwindef::WORD;
use crate::shared::CWideString;

pub(crate) enum ResourceIDOrIDString {
    ID(WORD),
    String(CWideString),
}

impl ResourceIDOrIDString {
    pub(crate) fn as_ptr_or_int_ptr(&self) -> *const u16 {
        use winapi::um::winuser::MAKEINTRESOURCEW;
        match self {
            ResourceIDOrIDString::ID(id) => MAKEINTRESOURCEW(*id),
            ResourceIDOrIDString::String(str) => str.as_ptr(),
        }
    }
}