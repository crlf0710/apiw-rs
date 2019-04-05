use log::warn;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WORD;
use winapi::shared::windef::HMENU;

use crate::shared::booleanize;
use crate::shared::strategy;
use crate::shared::OkOrLastError;
use crate::shared::{Error, Result};
use crate::shared::{ManagedData, ManagedEntity, ManagedStrategy};
use crate::windows_subsystem::window::AnyWindow;

#[derive(Clone)]
pub struct MenuInner(HMENU);

impl MenuInner {
    pub(crate) fn raw_handle(&self) -> HMENU {
        self.0
    }
}

impl ManagedData for MenuInner {
    fn share(&self) -> Self {
        MenuInner(self.0)
    }
    fn delete(&mut self) {
        use winapi::um::winuser::DestroyMenu;
        unsafe {
            let succeeded = booleanize(DestroyMenu(self.raw_handle()));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "AnyMenu", Error::last::<()>());
            }
        }
    }
}

pub type AnyMenu<T> = ManagedEntity<MenuInner, T>;
pub type ForeignMenu = AnyMenu<strategy::Foreign>;

impl<T: ManagedStrategy> AnyWindow<T> {
    pub fn has_menu(&self) -> Result<bool> {
        use winapi::um::winuser::GetMenu;
        let has_window_menu = unsafe {
            let h = GetMenu(self.data_ref().raw_handle()).ok_or_last_error()?;
            !h.is_null()
        };
        Ok(has_window_menu)
    }

    pub fn menu(&self) -> Result<Option<ForeignMenu>> {
        use winapi::um::winuser::GetMenu;
        let menu = unsafe {
            let h = GetMenu(self.data_ref().raw_handle()).ok_or_last_error()?;
            h
        };
        Ok(ForeignMenu::new_from_attached(menu))
    }
}

impl<T: ManagedStrategy> AnyMenu<T> {
    pub fn new_from_attached(h: HMENU) -> Option<ForeignMenu> {
        if h.is_null() {
            return None;
        }
        Some(strategy::Foreign::attached_entity(MenuInner(h)))
    }

    pub fn item_by_command(&mut self, command: WORD) -> MenuItem {
        MenuItem {
            menu: self.data_mut(),
            by_command: true,
            id_or_pos: command as _,
        }
    }
}

pub struct MenuItem<'a> {
    menu: &'a mut MenuInner,
    by_command: bool,
    id_or_pos: UINT,
}

impl<'a> MenuItem<'a> {
    pub fn set_checked(&mut self, checked: bool) -> Result<&mut Self> {
        use winapi::um::winuser::CheckMenuItem;
        use winapi::um::winuser::MF_BYCOMMAND;
        use winapi::um::winuser::MF_BYPOSITION;
        use winapi::um::winuser::MF_CHECKED;
        use winapi::um::winuser::MF_UNCHECKED;
        unsafe {
            let h = self.menu.0;
            let mut f = 0;
            if self.by_command {
                f |= MF_BYCOMMAND;
            } else {
                f |= MF_BYPOSITION;
            }
            if checked {
                f |= MF_CHECKED;
            } else {
                f |= MF_UNCHECKED;
            }
            let r = CheckMenuItem(h, self.id_or_pos as _, f);
            if r == -1i32 as _ {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<&mut Self> {
        use winapi::um::winuser::EnableMenuItem;
        use winapi::um::winuser::MF_BYCOMMAND;
        use winapi::um::winuser::MF_BYPOSITION;
        use winapi::um::winuser::MF_ENABLED;
        use winapi::um::winuser::MF_GRAYED;
        unsafe {
            let h = self.menu.0;
            let mut f = 0;
            if self.by_command {
                f |= MF_BYCOMMAND;
            } else {
                f |= MF_BYPOSITION;
            }
            if enabled {
                f |= MF_ENABLED;
            } else {
                f |= MF_GRAYED;
            }
            let r = EnableMenuItem(h, self.id_or_pos as _, f);
            if r == -1i32 as _ {
                return Error::last();
            }
        }
        Ok(self)
    }

    pub fn set_enabled_but_never_grayed(&mut self, enabled: bool) -> Result<&mut Self> {
        use winapi::um::winuser::EnableMenuItem;
        use winapi::um::winuser::MF_BYCOMMAND;
        use winapi::um::winuser::MF_BYPOSITION;
        use winapi::um::winuser::MF_DISABLED;
        use winapi::um::winuser::MF_ENABLED;
        unsafe {
            let h = self.menu.0;
            let mut f = 0;
            if self.by_command {
                f |= MF_BYCOMMAND;
            } else {
                f |= MF_BYPOSITION;
            }
            if enabled {
                f |= MF_ENABLED;
            } else {
                f |= MF_DISABLED;
            }
            let r = EnableMenuItem(h, self.id_or_pos as _, f);
            if r == -1i32 as _ {
                return Error::last();
            }
        }
        Ok(self)
    }
}
