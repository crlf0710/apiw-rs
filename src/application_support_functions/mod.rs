use num_traits::FromPrimitive;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::DWORD;
use wio::error::last_error;
use wio::Result;

use std::path::PathBuf;

use shared::CWideString;
use shared::CWideStringSeq;
use shared::ManagedStrategy;
use shared::booleanize;
use shared::CommDlgResult;
use windows_subsystem::window::WindowInner;
use windows_subsystem::window::AnyWindow;

pub struct MessageBoxBuilder<'a> {
    parent: Option<&'a WindowInner>,
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
        use std::ptr::null_mut;
        use winapi::um::winuser::MessageBoxW;
        let r = unsafe {
            MessageBoxW(
                self.parent.map_or_else(null_mut, WindowInner::raw_handle),
                self.message.as_ptr(),
                self.title.as_ptr(),
                self.style,
            )
        };
        if let Some(result) = MessageBoxResult::from_i32(r) {
            Ok(result)
        } else {
            last_error()
        }
    }
}

#[derive(BitOr)]
pub struct OpenFileDialogFlags(DWORD);

impl OpenFileDialogFlags {
    pub const SHOW_HELP: OpenFileDialogFlags = OpenFileDialogFlags(::winapi::um::commdlg::OFN_SHOWHELP);
    pub const EXPLORER: OpenFileDialogFlags = OpenFileDialogFlags(::winapi::um::commdlg::OFN_EXPLORER);
    pub const FILE_MUST_EXIST: OpenFileDialogFlags = OpenFileDialogFlags(::winapi::um::commdlg::OFN_FILEMUSTEXIST);
    pub const PATH_MUST_EXIST: OpenFileDialogFlags = OpenFileDialogFlags(::winapi::um::commdlg::OFN_PATHMUSTEXIST);
}

#[derive(BitOr)]
pub struct SaveFileDialogFlags(DWORD);

impl SaveFileDialogFlags {
    pub const SHOW_HELP: SaveFileDialogFlags = SaveFileDialogFlags(::winapi::um::commdlg::OFN_SHOWHELP);
    pub const EXPLORER: SaveFileDialogFlags = SaveFileDialogFlags(::winapi::um::commdlg::OFN_EXPLORER);
    pub const PATH_MUST_EXIST: SaveFileDialogFlags = SaveFileDialogFlags(::winapi::um::commdlg::OFN_PATHMUSTEXIST);
}

/*
pub const OFN_READONLY: DWORD = 0x00000001;
pub const OFN_OVERWRITEPROMPT: DWORD = 0x00000002;
pub const OFN_HIDEREADONLY: DWORD = 0x00000004;
pub const OFN_NOCHANGEDIR: DWORD = 0x00000008;
pub const OFN_ENABLEHOOK: DWORD = 0x00000020;
pub const OFN_ENABLETEMPLATE: DWORD = 0x00000040;
pub const OFN_ENABLETEMPLATEHANDLE: DWORD = 0x00000080;
pub const OFN_NOVALIDATE: DWORD = 0x00000100;
pub const OFN_ALLOWMULTISELECT: DWORD = 0x00000200;
pub const OFN_EXTENSIONDIFFERENT: DWORD = 0x00000400;
pub const OFN_CREATEPROMPT: DWORD = 0x00002000;
pub const OFN_SHAREAWARE: DWORD = 0x00004000;
pub const OFN_NOREADONLYRETURN: DWORD = 0x00008000;
pub const OFN_NOTESTFILECREATE: DWORD = 0x00010000;
pub const OFN_NONETWORKBUTTON: DWORD = 0x00020000;
pub const OFN_NOLONGNAMES: DWORD = 0x00040000;
pub const OFN_NODEREFERENCELINKS: DWORD = 0x00100000;
pub const OFN_LONGNAMES: DWORD = 0x00200000;
pub const OFN_ENABLEINCLUDENOTIFY: DWORD = 0x00400000;
pub const OFN_ENABLESIZING: DWORD = 0x00800000;
pub const OFN_DONTADDTORECENT: DWORD = 0x02000000;
pub const OFN_FORCESHOWHIDDEN: DWORD = 0x10000000;
pub const OFN_EX_NOPLACESBAR: DWORD = 0x00000001;
pub const OFN_SHAREFALLTHROUGH: UINT_PTR = 2;
pub const OFN_SHARENOWARN: UINT_PTR = 1;
pub const OFN_SHAREWARN: UINT_PTR = 0;
*/

pub struct OpenFileDialogBuilder<'b> {
    parent: Option<&'b WindowInner>,
    default_extension: Option<CWideString>,
    flags: OpenFileDialogFlags,
}

impl<'b> OpenFileDialogBuilder<'b> {
    pub fn new() -> Self {
        OpenFileDialogBuilder {
            parent: None,
            default_extension: None,
            flags: OpenFileDialogFlags(0)
        }
    }

    pub fn parent(mut self, parent: &'b AnyWindow<impl ManagedStrategy>) -> Self {
        self.parent = Some(parent.data_ref());
        self
    }

    pub fn default_extension(mut self, default_ext: &str) -> Self {
        self.default_extension = Some(CWideString::from(default_ext));
        self
    }

    pub fn flags(mut self, flags: OpenFileDialogFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn show_dialog(mut self) -> CommDlgResult<Option<PathBuf>> {
        use winapi::um::commdlg::GetOpenFileNameW;
        use winapi::um::commdlg::OPENFILENAMEW;
        use winapi::shared::minwindef::MAX_PATH;
        use winapi::um::commdlg::CommDlgExtendedError;
        use shared::CommDlgErr;
        use wio::wide::FromWide;
        use std::mem::zeroed;
        use std::mem::size_of_val;
        unsafe {
            const BUFFER_SIZE: usize = MAX_PATH as usize;
            let mut output_string = vec![0u16; BUFFER_SIZE + 1];
            let mut ofn: OPENFILENAMEW = zeroed();
            ofn.lStructSize = size_of_val(&ofn) as _;
            ofn.lpstrFile = output_string.as_mut_ptr();
            ofn.nMaxFile = output_string.len() as _;
            if let Some(parent) = self.parent.as_ref() {
                ofn.hwndOwner = parent.raw_handle();
            }
            if let Some(default_ext) = self.default_extension.as_ref() {
                ofn.lpstrDefExt = default_ext.as_ptr();
            }
            ofn.Flags = self.flags.0;

            if booleanize(GetOpenFileNameW(&mut ofn)) {
                let mut multi_string = CWideStringSeq::from_raw_unchecked(output_string);
                if let Some(bytes) = multi_string.iter_wide_null().next() {
                    return Ok(Some(PathBuf::from_wide_null(bytes)));
                } else {
                    return Ok(None);
                }
            } else {
                let err_code = CommDlgExtendedError();
                if err_code == 0 {
                    return Ok(None);
                } else {
                    return Err(CommDlgErr(err_code));
                }
            }
        }
    }
}
/*
.default_extension(default_ext)
.flags(OpenFileDialogFlags::SHOW_HELP |
OpenFileDialogFlags::PATH_MUST_EXIST |
OpenFileDialogFlags::FILE_MUST_EXIST |
OpenFileDialogFlags::EXPLORER
)
.show_dialog()
*/


pub struct SaveFileDialogBuilder<'b> {
    parent: Option<&'b WindowInner>,
    default_extension: Option<CWideString>,
    flags: SaveFileDialogFlags,
}

impl<'b> SaveFileDialogBuilder<'b> {
    pub fn new() -> Self {
        SaveFileDialogBuilder {
            parent: None,
            default_extension: None,
            flags: SaveFileDialogFlags(0)
        }
    }

    pub fn parent(mut self, parent: &'b AnyWindow<impl ManagedStrategy>) -> Self {
        self.parent = Some(parent.data_ref());
        self
    }

    pub fn default_extension(mut self, default_ext: &str) -> Self {
        self.default_extension = Some(CWideString::from(default_ext));
        self
    }

    pub fn flags(mut self, flags: SaveFileDialogFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn show_dialog(mut self) -> CommDlgResult<Option<PathBuf>> {
        use winapi::um::commdlg::GetSaveFileNameW;
        use winapi::um::commdlg::OPENFILENAMEW;
        use winapi::shared::minwindef::MAX_PATH;
        use winapi::um::commdlg::CommDlgExtendedError;
        use shared::CommDlgErr;
        use wio::wide::FromWide;
        use std::mem::zeroed;
        use std::mem::size_of_val;
        unsafe {
            const BUFFER_SIZE: usize = MAX_PATH as usize;
            let mut output_string = vec![0u16; BUFFER_SIZE + 1];
            let mut ofn: OPENFILENAMEW = zeroed();
            ofn.lStructSize = size_of_val(&ofn) as _;
            ofn.lpstrFile = output_string.as_mut_ptr();
            ofn.nMaxFile = output_string.len() as _;
            if let Some(parent) = self.parent.as_ref() {
                ofn.hwndOwner = parent.raw_handle();
            }
            if let Some(default_ext) = self.default_extension.as_ref() {
                ofn.lpstrDefExt = default_ext.as_ptr();
            }
            ofn.Flags = self.flags.0;

            if booleanize(GetSaveFileNameW(&mut ofn)) {
                let mut multi_string = CWideStringSeq::from_raw_unchecked(output_string);
                if let Some(bytes) = multi_string.iter_wide_null().next() {
                    return Ok(Some(PathBuf::from_wide_null(bytes)));
                } else {
                    return Ok(None);
                }
            } else {
                let err_code = CommDlgExtendedError();
                if err_code == 0 {
                    return Ok(None);
                } else {
                    return Err(CommDlgErr(err_code));
                }
            }
        }
    }
}