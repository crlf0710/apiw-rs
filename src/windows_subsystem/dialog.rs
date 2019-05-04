use winapi::shared::basetsd::INT_PTR;
use winapi::shared::minwindef::{WORD, UINT, WPARAM, LPARAM, HINSTANCE};
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::HANDLE;
use winapi::um::winuser::DLGPROC;
use crate::windows_subsystem::ResourceIDOrIDString;
use crate::windows_subsystem::window::{WindowInner, AnyWindow};
use crate::shared::ManagedStrategy;
use crate::{Result, Error, maybe_last_error};

pub struct DialogParam(LPARAM);

pub struct DialogTemplate(HANDLE);

impl DialogTemplate {
    fn as_ptr(&self) -> winapi::um::winuser::LPCDLGTEMPLATEW {
        unimplemented!()
    }
}

pub struct DialogResult(INT_PTR);

enum DialogBuilderTemplate {
    Resource(ResourceIDOrIDString),
    MemTemplate(DialogTemplate)
}

pub struct DialogBuilder<'a> {
    template: DialogBuilderTemplate,
    parent: Option<&'a WindowInner>,
    dlgproc: Option<DLGPROC>,
    param: Option<DialogParam>,
}

impl<'a> DialogBuilder<'a> {
    pub fn new_from_resource_id(id: WORD) -> Self {
        DialogBuilder {
            template: DialogBuilderTemplate::Resource(ResourceIDOrIDString::ID(id)),
            parent: None,
            dlgproc: None,
            param: None
        }
    }

    pub fn parent(mut self, parent: &'a AnyWindow<impl ManagedStrategy>) -> Self {
        self.parent = Some(parent.data_ref());
        self
    }

    pub fn param(mut self, param: DialogParam) -> Self {
        self.param = Some(param);
        self
    }

    pub fn dialog_proc(mut self, dialog_proc: DLGPROC) -> Self {
        self.dlgproc = Some(dialog_proc);
        self
    }

    unsafe extern "system" fn default_dialog_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, _: LPARAM) -> INT_PTR {
        use winapi::um::winuser::WM_COMMAND;
        use winapi::shared::minwindef::LOWORD;
        use winapi::um::winuser::{IDOK, IDCANCEL};
        use winapi::um::winuser::EndDialog;
        if msg == WM_COMMAND {
            let id = LOWORD(wparam as u32);
            if id == IDOK as _ || id == IDCANCEL as _ {
                unsafe {
                    EndDialog(hwnd, id as INT_PTR);
                }
                return 1;
            }
        }

        0
    }

    pub fn invoke(self) -> Result<DialogResult> {
        use std::ptr::null_mut;
        use winapi::um::winuser::{DialogBoxParamW, DialogBoxIndirectParamW};
        let result = unsafe {
            let h = match self.template {
                DialogBuilderTemplate::Resource(res) => {
                    DialogBoxParamW(crate::shared::exe_instance(), 
                        res.as_ptr_or_int_ptr(),
                        self.parent.map_or_else(null_mut, WindowInner::raw_handle),
                        self.dlgproc.unwrap_or(Some(Self::default_dialog_proc)),
                        self.param.map_or(0, |x| x.0))
                },
                DialogBuilderTemplate::MemTemplate(mem_template) => {
                    DialogBoxIndirectParamW(crate::shared::exe_instance(),
                        mem_template.as_ptr(),
                        self.parent.map_or_else(null_mut, WindowInner::raw_handle),
                        self.dlgproc.unwrap_or(Some(Self::default_dialog_proc)),
                        self.param.map_or(0, |x| x.0))
                }
            };
            if h == 0 || h == -1 {
                maybe_last_error(||())?;
            }
            h
        };
        Ok(DialogResult(result))
    }
}
