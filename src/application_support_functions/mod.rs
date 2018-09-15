use winapi::shared::minwindef::UINT;
use wio::Result;
use wio::error::last_error;
use num_traits::FromPrimitive;

use windows_subsystem::window::Window;
use utils::CWideString;

pub struct MessageBoxBuilder<'a> {
    parent: Option<&'a Window>,
    message: CWideString,
    title: CWideString,
    style: UINT,
}

#[repr(i32)]
#[derive(Primitive)]
pub enum MessageBoxResult {
    OK = ::winapi::um::winuser::IDOK,
    YES = ::winapi::um::winuser::IDYES,
    NO = ::winapi::um::winuser::IDNO,
    ABORT = ::winapi::um::winuser::IDABORT,
    RETRY = ::winapi::um::winuser::IDRETRY,
    IGNORE = ::winapi::um::winuser::IDIGNORE,
    CANCEL = ::winapi::um::winuser::IDCANCEL,
}

impl<'a> MessageBoxBuilder<'a> {
    pub fn new() -> Self {
        MessageBoxBuilder {
            parent: None,
            message: CWideString::new(),
            title: CWideString::new(),
            style: 0,
        }
    }

    pub fn message(mut self, v: &str) -> Self {
        self.message = v.into();
        self
    }

    pub fn title(mut self, v: &str) -> Self {
        self.title = v.into();
        self
    }

    /// ECMA-234 Clause 434 MessageBox
    pub fn invoke(self) -> Result<MessageBoxResult> {
        use winapi::um::winuser::MessageBoxW;
        use std::ptr::null_mut;
        let r = unsafe {
            MessageBoxW(
                self.parent.map_or_else(null_mut, Window::raw_handle),
                self.message.as_ptr(),
                self.title.as_ptr(),
                self.style
            )
        };
        if let Some(result) = MessageBoxResult::from_i32(r) {
            Ok(result)
        } else {
            last_error()
        }
    }
}

