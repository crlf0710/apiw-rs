#![cfg(debug_assertions)]
use crate::windows_subsystem::window::AnyWindow;
use crate::shared::ManagedStrategy;
use winapi::shared::windef::HWND;

impl<T: ManagedStrategy> AnyWindow<T> {
    pub fn raw_handle(&self) -> HWND {
        self.data_ref().raw_handle()
    }
}
