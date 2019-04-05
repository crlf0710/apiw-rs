#[derive(PartialEq)]
struct GraphicsMode(winapi::ctypes::c_int);

impl GraphicsMode {
    const COMPATIBLE: GraphicsMode = GraphicsMode(winapi::um::wingdi::GM_COMPATIBLE as _);
    const ADVANCED: GraphicsMode = GraphicsMode(winapi::um::wingdi::GM_ADVANCED as _);
}

/*
impl<'a> ScopedDeviceContext<'a> {
    pub fn set_graphics_mode(&mut self, graphics_mode: GraphicsMode) -> Result<&mut Self> {

    }
}
*/