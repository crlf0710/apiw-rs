#![cfg(debug_assertions)]
use crate::shared::ManagedStrategy;
use crate::windows_subsystem::window::AnyWindow;
use winapi::shared::windef::HWND;

impl<T: ManagedStrategy> AnyWindow<T> {
    pub fn raw_handle(&self) -> HWND {
        self.data_ref().raw_handle()
    }
}
