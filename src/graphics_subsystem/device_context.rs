use winapi::shared::windef::HDC;
use wio::error::last_error;

use utils::booleanize;
use utils::Handle;

pub struct DeviceContext(pub(crate) HDC);

impl DeviceContext {
    pub fn raw_handle(&self) -> HDC {
        self.0
    }
}

impl Handle for DeviceContext {
    fn duplicate(&self) -> Self {
        DeviceContext(self.0)
    }

    fn clean_up(&mut self) {
        use winapi::um::wingdi::DeleteDC;
        unsafe {
            let succeeded = booleanize(DeleteDC(self.raw_handle()));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "DeviceContext", last_error::<()>());
            }
        }
    }
}
