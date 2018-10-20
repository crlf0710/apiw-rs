use winapi;
use winapi::ctypes::c_int;
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::{ATOM, HINSTANCE};
use winapi::shared::minwindef::{DWORD, LPVOID, UINT, WORD};
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HBRUSH;
use winapi::shared::windef::HCURSOR;
use winapi::shared::windef::HICON;
use winapi::shared::windef::HMENU;
use winapi::shared::windef::HWND;
use winapi::um::winuser::WNDPROC;
use {last_error, maybe_last_error, Result};

use utils::booleanize;
use utils::exe_instance;
use utils::revert_booleanize;
use utils::CWideString;
use utils::ManagedStrategy;
//use utils::{Handle, Managed, Temporary};
//use utils::System;
use graphics_subsystem::Rect;
use graphics_subsystem::Size;
use utils::strategy;
use utils::ManagedData;
use utils::ManagedEntity;
use utils::OkOrLastError;

pub type AnyWindowClass<T> = ManagedEntity<WindowClassInner, T>;
pub type ForeignWindowClass = AnyWindowClass<strategy::Foreign>;

#[derive(Clone)]
pub enum WindowClassInner {
    Atom(ATOM),
    String(CWideString),
}

impl WindowClassInner {
    fn as_ptr_or_atom_ptr(&self) -> *const u16 {
        match &self {
            WindowClassInner::Atom(atom) => *atom as _,
            WindowClassInner::String(str) => str.as_ptr(),
        }
    }
}

impl ManagedData for WindowClassInner {
    fn share(&self) -> Self {
        self.clone()
    }

    fn delete(&mut self) {
        use winapi::um::winuser::UnregisterClassW;
        unsafe {
            let succeeded = booleanize(UnregisterClassW(self.as_ptr_or_atom_ptr(), exe_instance()));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "WindowClass", last_error::<()>());
            }
        }
    }
}

impl ForeignWindowClass {
    fn new_with_atom(v: ATOM) -> ForeignWindowClass {
        strategy::Foreign::attached_entity(WindowClassInner::Atom(v))
    }
}

impl<T: ManagedStrategy> ManagedEntity<WindowClassInner, T> {
    pub(crate) fn as_ptr_or_atom_ptr(&self) -> *const u16 {
        self.data_ref().as_ptr_or_atom_ptr()
    }
}

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

#[derive(Into)]
pub struct SysColor(c_int);

impl SysColor {
    pub const THREE_DIM_DARK_SHADOW: SysColor = SysColor(::winapi::um::winuser::COLOR_3DDKSHADOW);
    pub const THREE_DIM_FACE: SysColor = SysColor(::winapi::um::winuser::COLOR_3DFACE);
    pub const THREE_DIM_HIGHLIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_3DHIGHLIGHT);
    pub const THREE_DIM_HILIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_3DHILIGHT);
    pub const THREE_DIM_LIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_3DLIGHT);
    pub const THREE_DIM_SHADOW: SysColor = SysColor(::winapi::um::winuser::COLOR_3DSHADOW);
    pub const ACTIVE_BORDER: SysColor = SysColor(::winapi::um::winuser::COLOR_ACTIVEBORDER);
    pub const ACTIVE_CAPTION: SysColor = SysColor(::winapi::um::winuser::COLOR_ACTIVECAPTION);
    pub const APP_WORKSPACE: SysColor = SysColor(::winapi::um::winuser::COLOR_APPWORKSPACE);
    pub const BACKGROUND: SysColor = SysColor(::winapi::um::winuser::COLOR_BACKGROUND);
    pub const BUTTON_FACE: SysColor = SysColor(::winapi::um::winuser::COLOR_BTNFACE);
    pub const BUTTON_HIGHLIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_BTNHIGHLIGHT);
    pub const BUTTON_HILIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_BTNHILIGHT);
    pub const BUTTON_SHADOW: SysColor = SysColor(::winapi::um::winuser::COLOR_BTNSHADOW);
    pub const BUTTON_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_BTNTEXT);
    pub const CAPTION_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_CAPTIONTEXT);
    pub const DESKTOP: SysColor = SysColor(::winapi::um::winuser::COLOR_DESKTOP);
    pub const GRADIENT_ACTIVE_CAPTION: SysColor =
        SysColor(::winapi::um::winuser::COLOR_GRADIENTACTIVECAPTION);
    pub const GRADIENT_INACTIVE_CAPTION: SysColor =
        SysColor(::winapi::um::winuser::COLOR_GRADIENTINACTIVECAPTION);
    pub const GRAY_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_GRAYTEXT);
    pub const HIGHLIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_HIGHLIGHT);
    pub const HIGHLIGHT_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_HIGHLIGHTTEXT);
    pub const HOTLIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_HOTLIGHT);
    pub const INACTIVE_BORDER: SysColor = SysColor(::winapi::um::winuser::COLOR_INACTIVEBORDER);
    pub const INACTIVE_CAPTION: SysColor = SysColor(::winapi::um::winuser::COLOR_INACTIVECAPTION);
    pub const INACTIVE_CAPTION_TEXT: SysColor =
        SysColor(::winapi::um::winuser::COLOR_INACTIVECAPTIONTEXT);
    pub const INFO_BACKGROUND: SysColor = SysColor(::winapi::um::winuser::COLOR_INFOBK);
    pub const INFO_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_INFOTEXT);
    pub const MENU: SysColor = SysColor(::winapi::um::winuser::COLOR_MENU);
    pub const MENU_BAR: SysColor = SysColor(::winapi::um::winuser::COLOR_MENUBAR);
    pub const MENU_HILIGHT: SysColor = SysColor(::winapi::um::winuser::COLOR_MENUHILIGHT);
    pub const MENU_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_MENUTEXT);
    pub const SCROLL_BAR: SysColor = SysColor(::winapi::um::winuser::COLOR_SCROLLBAR);
    pub const WINDOW: SysColor = SysColor(::winapi::um::winuser::COLOR_WINDOW);
    pub const WINDOW_FRAME: SysColor = SysColor(::winapi::um::winuser::COLOR_WINDOWFRAME);
    pub const WINDOW_TEXT: SysColor = SysColor(::winapi::um::winuser::COLOR_WINDOWTEXT);
}

#[derive(Into)]
pub struct SysCursor(::winapi::shared::ntdef::LPCWSTR);

impl SysCursor {
    pub const ARROW: SysCursor = SysCursor(::winapi::um::winuser::IDC_ARROW);
    pub const I_BEAM: SysCursor = SysCursor(::winapi::um::winuser::IDC_IBEAM);
    pub const WAIT: SysCursor = SysCursor(::winapi::um::winuser::IDC_WAIT);
    pub const CROSS: SysCursor = SysCursor(::winapi::um::winuser::IDC_CROSS);
    pub const UPARROW: SysCursor = SysCursor(::winapi::um::winuser::IDC_UPARROW);
    pub const SIZE: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZE);
    pub const ICON: SysCursor = SysCursor(::winapi::um::winuser::IDC_ICON);
    pub const SIZE_NWSE: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZENWSE);
    pub const SIZE_NESW: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZENESW);
    pub const SIZE_WE: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZEWE);
    pub const SIZE_NS: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZENS);
    pub const SIZE_ALL: SysCursor = SysCursor(::winapi::um::winuser::IDC_SIZEALL);
    pub const NO: SysCursor = SysCursor(::winapi::um::winuser::IDC_NO);
    pub const HAND: SysCursor = SysCursor(::winapi::um::winuser::IDC_HAND);
    pub const APP_STARTING: SysCursor = SysCursor(::winapi::um::winuser::IDC_APPSTARTING);
    pub const HELP: SysCursor = SysCursor(::winapi::um::winuser::IDC_HELP);
}

enum OwnedBrushOrSystemColor {
    OwnedBrush(HBRUSH),
    SystemColor(c_int),
}

// FIXME: Drop issue.
impl OwnedBrushOrSystemColor {
    fn as_brush_or_int_brush(&self) -> HBRUSH {
        match self {
            OwnedBrushOrSystemColor::OwnedBrush(brush) => *brush,
            OwnedBrushOrSystemColor::SystemColor(clr) => (*clr + 1) as usize as _,
        }
    }
}

pub struct WindowClassBuilder {
    name: CWideString,
    style: UINT,
    instance: HINSTANCE,
    class_extra_size: c_int,
    window_extra_size: c_int,
    icon: (Option<HICON>, Option<HICON>),
    background_brush: Option<OwnedBrushOrSystemColor>,
    cursor: Option<HCURSOR>,
    menu: Option<ResourceIDOrIDString>,
    window_proc: WNDPROC,
}

type WndProcInner = unsafe extern "system" fn(_: HWND, _: UINT, _: WPARAM, _: LPARAM) -> LRESULT;

impl WindowClassBuilder {
    pub fn new(name: &str) -> Self {
        WindowClassBuilder {
            name: name.into(),
            style: 0,
            instance: exe_instance(),
            class_extra_size: 0,
            window_extra_size: 0,
            icon: (None, None),
            background_brush: None,
            cursor: None,
            menu: None,
            window_proc: Some(::winapi::um::winuser::DefWindowProcW),
        }
    }

    pub fn window_proc(mut self, wnd_proc: WndProcInner) -> Self {
        self.window_proc = Some(wnd_proc);
        self
    }

    pub fn syscolor_background_brush(mut self, syscolor: SysColor) -> Self {
        self.background_brush = Some(OwnedBrushOrSystemColor::SystemColor(syscolor.into()));
        self
    }

    pub fn syscursor(mut self, syscursor: SysCursor) -> Self {
        use std::ptr::null_mut;
        use winapi::um::winuser::LoadCursorW;

        self.cursor = Some(unsafe {
            //FIXME is this properly released?
            LoadCursorW(null_mut(), syscursor.into())
        });
        self
    }

    pub fn create_managed(self) -> Result<ForeignWindowClass> {
        use std::ptr::{null, null_mut};
        use winapi::um::winuser::RegisterClassExW;
        use winapi::um::winuser::WNDCLASSEXW;

        let window_class = unsafe {
            let wcex = WNDCLASSEXW {
                cbSize: ::std::mem::size_of::<WNDCLASSEXW>() as _,
                style: self.style,
                hInstance: self.instance,
                cbClsExtra: self.class_extra_size,
                cbWndExtra: self.window_extra_size,
                hIcon: self.icon.0.unwrap_or_else(null_mut),
                hIconSm: self.icon.1.unwrap_or_else(null_mut),
                hbrBackground: self
                    .background_brush
                    .as_ref()
                    .map_or_else(null_mut, OwnedBrushOrSystemColor::as_brush_or_int_brush),
                hCursor: self.cursor.unwrap_or_else(null_mut),
                lpszClassName: self.name.as_ptr(),
                lpszMenuName: self
                    .menu
                    .as_ref()
                    .map_or_else(null, ResourceIDOrIDString::as_ptr_or_int_ptr),
                lpfnWndProc: self.window_proc,
            };
            let h = RegisterClassExW(&wcex);
            if h == 0 {
                return last_error();
            }
            h
        };
        Ok(ForeignWindowClass::new_with_atom(window_class))
    }
}

/*
	WNDCLASSEX wcex;

	wcex.cbSize = sizeof(WNDCLASSEX);

	wcex.style			= CS_HREDRAW | CS_VREDRAW;
	wcex.lpfnWndProc	= WndProc;
	wcex.cbClsExtra		= 0;
	wcex.cbWndExtra		= 0;
	wcex.hInstance		= hInstance;
	wcex.hIcon			= LoadIcon(hInstance, MAKEINTRESOURCE(IDI_CHARLESMINE));
	wcex.hCursor		= LoadCursor(NULL, IDC_ARROW);
	wcex.hbrBackground	= (HBRUSH)(COLOR_BTNFACE+1);
	wcex.lpszMenuName	= MAKEINTRESOURCE(IDC_CHARLESMINE);
	wcex.lpszClassName	= szWindowClass;
	wcex.hIconSm		= wcex.hIcon;

	return RegisterClassEx(&wcex);
    */

enum MenuOrChildWindowId {
    Menu(HMENU),
    ChildWindowId(WORD),
}

impl MenuOrChildWindowId {
    fn as_either_ptr(&self) -> HMENU {
        match self {
            MenuOrChildWindowId::Menu(menu) => *menu,
            MenuOrChildWindowId::ChildWindowId(id) => *id as usize as _,
        }
    }
}

pub type AnyWindow<T> = ManagedEntity<WindowInner, T>;
pub type ForeignWindow = AnyWindow<strategy::Foreign>;

#[derive(Clone)]
pub struct WindowInner(HWND);

impl WindowInner {
    pub(crate) fn raw_handle(&self) -> HWND {
        self.0
    }
}

impl ManagedData for WindowInner {
    fn share(&self) -> Self {
        WindowInner(self.0)
    }
    fn delete(&mut self) {
        use winapi::um::winuser::DestroyWindow;
        unsafe {
            let succeeded = booleanize(DestroyWindow(self.raw_handle()));
            if !succeeded {
                warn!(target: "apiw", "Failed to cleanup {}, last error: {:?}", "AnyWindow", last_error::<()>());
            }
        }
    }
}

pub struct WindowBuilder<'a, 'b> {
    class: &'a WindowClassInner,
    parent: Option<&'b WindowInner>,
    name: Option<CWideString>,
    menu: Option<MenuOrChildWindowId>,
    instance: HINSTANCE,
    style: (DWORD, DWORD),
    position: Option<(c_int, c_int)>,
    size: Option<(c_int, c_int)>,
    param: LPVOID,
}

impl<'a, 'b> WindowBuilder<'a, 'b> {
    pub fn new(window_class: &'a AnyWindowClass<impl ManagedStrategy>) -> Self {
        Self {
            class: window_class.data_ref(),
            parent: None,
            name: None,
            menu: None,
            instance: exe_instance(),
            style: (0, 0),
            position: None,
            size: None,
            param: 0usize as _,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn style(mut self, style: WindowStyles) -> Self {
        self.style.0 = style.bits();
        self
    }

    /// ECMA-234 Clause 27 CreateWindow CreateWindowEx
    pub fn create(self) -> Result<AnyWindow<strategy::Foreign>> {
        use std::ptr::{null, null_mut};
        use winapi::um::winuser::CreateWindowExW;
        use winapi::um::winuser::CW_USEDEFAULT;
        let window = unsafe {
            let position = self.position.clone().unwrap_or((CW_USEDEFAULT, 0));
            let size = self.size.clone().unwrap_or((CW_USEDEFAULT, 0));
            let h = CreateWindowExW(
                self.style.1,
                self.class.as_ptr_or_atom_ptr(),
                self.name.as_ref().map_or_else(null, CWideString::as_ptr),
                self.style.0,
                position.0,
                position.1,
                size.0,
                size.1,
                self.parent.map_or_else(null_mut, |v| v.0),
                self.menu
                    .as_ref()
                    .map_or_else(null_mut, MenuOrChildWindowId::as_either_ptr),
                self.instance,
                self.param,
            );
            if h.is_null() {
                return last_error();
            };
            h
        };
        AnyWindow::new_from_attached(window).ok_or_last_error()
    }
}

bitflags! {
    pub struct WindowStyles : DWORD {
        const OVERLAPPED = ::winapi::um::winuser::WS_OVERLAPPED;
        const POPUP = ::winapi::um::winuser::WS_POPUP;
        const CHILD = ::winapi::um::winuser::WS_CHILD;
        const MINIMIZE = ::winapi::um::winuser::WS_MINIMIZE;
        const VISIBLE = ::winapi::um::winuser::WS_VISIBLE;
        const DISABLED = ::winapi::um::winuser::WS_DISABLED;
        const CLIPSIBLINGS = ::winapi::um::winuser::WS_CLIPSIBLINGS;
        const CLIPCHILDREN = ::winapi::um::winuser::WS_CLIPCHILDREN;
        const MAXIMIZE = ::winapi::um::winuser::WS_MAXIMIZE;
        const CAPTION = ::winapi::um::winuser::WS_CAPTION;
        const BORDER = ::winapi::um::winuser::WS_BORDER;
        const DLGFRAME = ::winapi::um::winuser::WS_DLGFRAME;
        const VSCROLL = ::winapi::um::winuser::WS_VSCROLL;
        const HSCROLL = ::winapi::um::winuser::WS_HSCROLL;
        const SYSMENU = ::winapi::um::winuser::WS_SYSMENU;
        const THICKFRAME = ::winapi::um::winuser::WS_THICKFRAME;
        const GROUP = ::winapi::um::winuser::WS_GROUP;
        const TABSTOP = ::winapi::um::winuser::WS_TABSTOP;
        const MINIMIZEBOX = ::winapi::um::winuser::WS_MINIMIZEBOX;
        const MAXIMIZEBOX = ::winapi::um::winuser::WS_MAXIMIZEBOX;
        const TILED = ::winapi::um::winuser::WS_TILED;
        const ICONIC = ::winapi::um::winuser::WS_ICONIC;
        const SIZEBOX = ::winapi::um::winuser::WS_SIZEBOX;
        const TILEDWINDOW = ::winapi::um::winuser::WS_TILEDWINDOW;
        const OVERLAPPEDWINDOW = ::winapi::um::winuser::WS_OVERLAPPEDWINDOW;
        const POPUPWINDOW = ::winapi::um::winuser::WS_POPUPWINDOW;
        const CHILDWINDOW = ::winapi::um::winuser::WS_CHILDWINDOW;
    }
}

bitflags! {
    pub struct WindowExtendedStyles : DWORD {
        const DLGMODALFRAME = ::winapi::um::winuser::WS_EX_DLGMODALFRAME;
        const NOPARENTNOTIFY = ::winapi::um::winuser::WS_EX_NOPARENTNOTIFY;
        const TOPMOST = ::winapi::um::winuser::WS_EX_TOPMOST;
        const ACCEPTFILES = ::winapi::um::winuser::WS_EX_ACCEPTFILES;
        const TRANSPARENT = ::winapi::um::winuser::WS_EX_TRANSPARENT;
        const MDICHILD = ::winapi::um::winuser::WS_EX_MDICHILD;
        const TOOLWINDOW = ::winapi::um::winuser::WS_EX_TOOLWINDOW;
        const WINDOWEDGE = ::winapi::um::winuser::WS_EX_WINDOWEDGE;
        const CLIENTEDGE = ::winapi::um::winuser::WS_EX_CLIENTEDGE;
        const CONTEXTHELP = ::winapi::um::winuser::WS_EX_CONTEXTHELP;
        const RIGHT = ::winapi::um::winuser::WS_EX_RIGHT;
        const LEFT = ::winapi::um::winuser::WS_EX_LEFT;
        const RTLREADING = ::winapi::um::winuser::WS_EX_RTLREADING;
        const LTRREADING = ::winapi::um::winuser::WS_EX_LTRREADING;
        const LEFTSCROLLBAR = ::winapi::um::winuser::WS_EX_LEFTSCROLLBAR;
        const RIGHTSCROLLBAR = ::winapi::um::winuser::WS_EX_RIGHTSCROLLBAR;
        const CONTROLPARENT = ::winapi::um::winuser::WS_EX_CONTROLPARENT;
        const STATICEDGE = ::winapi::um::winuser::WS_EX_STATICEDGE;
        const APPWINDOW = ::winapi::um::winuser::WS_EX_APPWINDOW;
        const OVERLAPPEDWINDOW = ::winapi::um::winuser::WS_EX_OVERLAPPEDWINDOW;
        const PALETTEWINDOW = ::winapi::um::winuser::WS_EX_PALETTEWINDOW;
        const LAYERED = ::winapi::um::winuser::WS_EX_LAYERED;
        const NOINHERITLAYOUT = ::winapi::um::winuser::WS_EX_NOINHERITLAYOUT;
        const NOREDIRECTIONBITMAP = ::winapi::um::winuser::WS_EX_NOREDIRECTIONBITMAP;
        const LAYOUTRTL = ::winapi::um::winuser::WS_EX_LAYOUTRTL;
        const COMPOSITED = ::winapi::um::winuser::WS_EX_COMPOSITED;
        const NOACTIVATE = ::winapi::um::winuser::WS_EX_NOACTIVATE;
    }
}

impl ForeignWindow {
    pub fn new_from_attached(h: HWND) -> Option<Self> {
        if h.is_null() {
            return None;
        }
        Some(strategy::Foreign::attached_entity(WindowInner(h)))
    }
}

impl<T: ManagedStrategy> AnyWindow<T> {
    /// ECMA-234 Clause 41 ShowWindow
    pub fn show(&self, cmd: c_int) -> Result<&Self> {
        let mut prev_state: bool = false;
        self.show_and_get_prev_state(cmd, &mut prev_state)
    }

    /// ECMA-234 Clause 41 ShowWindow
    pub fn show_and_get_prev_state(&self, cmd: c_int, prev_state: &mut bool) -> Result<&Self> {
        use winapi::um::winuser::ShowWindow;
        unsafe {
            let r = ShowWindow(self.data_ref().raw_handle(), cmd);
            *prev_state = booleanize(r);
        }
        Ok(self)
    }

    /// ECMA-234 Clause 41 IsWindowVisible
    pub fn is_visible(&self) -> Result<bool> {
        use winapi::um::winuser::IsWindowVisible;
        let v = unsafe { booleanize(IsWindowVisible(self.data_ref().raw_handle())) };
        Ok(v)
    }

    pub fn styles(&self) -> Result<WindowStyles> {
        use winapi::um::winuser::GetWindowLongPtrW;
        use winapi::um::winuser::GWL_STYLE;
        let window_styles = unsafe {
            let mut h = GetWindowLongPtrW(self.data_ref().raw_handle(), GWL_STYLE);

            if h == 0 {
                h = maybe_last_error(|| 0)?;
            }
            h
        };
        Ok(WindowStyles::from_bits_truncate(window_styles as _))
    }

    pub fn extended_styles(&self) -> Result<WindowExtendedStyles> {
        use winapi::um::winuser::GetWindowLongPtrW;
        use winapi::um::winuser::GWL_EXSTYLE;
        let window_extended_styles = unsafe {
            let mut h = GetWindowLongPtrW(self.data_ref().raw_handle(), GWL_EXSTYLE);

            if h == 0 {
                h = maybe_last_error(|| 0)?;
            }
            h
        };
        Ok(WindowExtendedStyles::from_bits_truncate(
            window_extended_styles as _,
        ))
    }

    pub fn has_menu(&self) -> Result<bool> {
        use winapi::um::winuser::GetMenu;
        let has_window_menu = unsafe {
            let h = GetMenu(self.data_ref().raw_handle());
            !h.is_null()
        };
        Ok(has_window_menu)
    }

    pub fn predict_window_rect_from_client_rect_and_window(
        rect: Rect,
        window: &Self,
    ) -> Result<Rect> {
        use winapi::um::winuser::AdjustWindowRectEx;
        let mut rect_data = rect.into();
        let styles = window.styles()?;
        let exstyles = window.extended_styles()?;
        let has_menu = if styles.contains(WindowStyles::CHILD) {
            false
        } else {
            window.has_menu()?
        };
        unsafe {
            if !booleanize(AdjustWindowRectEx(
                &mut rect_data,
                styles.bits(),
                revert_booleanize(has_menu),
                exstyles.bits(),
            )) {
                return last_error();
            }
        }
        Ok(rect_data.into())
    }

    pub fn reposition_set_size(&self, size: Size) -> Result<&Self> {
        use winapi::_core::ptr::null_mut;
        use winapi::um::winuser::SetWindowPos;
        use winapi::um::winuser::{
            SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW,
            SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER,
        };
        /*
        full:
        SWP_NOACTIVATE | SWP_NOCOPYBITS | SWP_NOMOVE | SWP_NOOWNERZORDER |
            SWP_NOREDRAW | SWP_NOSENDCHANGING | SWP_NOSIZE | SWP_NOZORDER
        duplicates:
        SWP_NOREPOSITION
        */
        let mut full_flags = SWP_NOACTIVATE
            | SWP_NOCOPYBITS
            | SWP_NOMOVE
            | SWP_NOOWNERZORDER
            | SWP_NOREDRAW
            | SWP_NOSENDCHANGING
            | SWP_NOSIZE
            | SWP_NOZORDER;
        full_flags &= !SWP_NOSIZE;

        unsafe {
            if !booleanize(SetWindowPos(
                self.data_ref().raw_handle(),
                null_mut(),
                -1,
                -1,
                size.0.cx,
                size.0.cy,
                full_flags,
            )) {
                return last_error();
            }
        }
        Ok(self)
    }
}

pub enum WindowProcResponse {
    Done(LRESULT),
    Fallback,
}

#[derive(Copy, Clone)]
pub struct WindowProcRequestArgs {
    pub msg: UINT,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
}

use graphics_subsystem::Point;

pub struct MouseEventArgs<'a>(pub &'a WindowProcRequestArgs);

#[repr(u32)]
pub enum MouseEventArgType {
    LeftButtonDown = winapi::um::winuser::WM_LBUTTONDOWN,
    LeftButtonUp = winapi::um::winuser::WM_LBUTTONUP,
    RightButtonDown = winapi::um::winuser::WM_RBUTTONDOWN,
    RightButtonUp = winapi::um::winuser::WM_RBUTTONUP,
    MiddleButtonDown = winapi::um::winuser::WM_MBUTTONDOWN,
    MiddleButtonUp = winapi::um::winuser::WM_MBUTTONUP,

    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    __non_exhuastive,
}

impl<'a> MouseEventArgs<'a> {
    pub fn kind(&self) -> Option<MouseEventArgType> {
        use winapi::um::winuser::{
            WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_RBUTTONDOWN,
            WM_RBUTTONUP,
        };

        match self.0.msg {
            WM_LBUTTONDOWN => Some(MouseEventArgType::LeftButtonDown),
            WM_LBUTTONUP => Some(MouseEventArgType::LeftButtonUp),
            WM_RBUTTONDOWN => Some(MouseEventArgType::RightButtonDown),
            WM_RBUTTONUP => Some(MouseEventArgType::RightButtonUp),
            WM_MBUTTONDOWN => Some(MouseEventArgType::MiddleButtonDown),
            WM_MBUTTONUP => Some(MouseEventArgType::MiddleButtonUp),
            _ => None,
        }
    }
    pub fn cursor_coordinate(&self) -> Option<Point> {
        use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
        use winapi::um::winuser::{WM_MOUSEACTIVATE, WM_MOUSELEAVE};
        match self.0.msg {
            WM_MOUSEACTIVATE | WM_MOUSELEAVE => None,
            _ => Some(Point::new(
                GET_X_LPARAM(self.0.lparam) as _,
                GET_Y_LPARAM(self.0.lparam) as _,
            )),
        }
    }
}

pub struct WindowProcRequest<'a> {
    pub hwnd: HWND,
    pub args: WindowProcRequestArgs,
    pub response: Option<&'a mut WindowProcResponse>,
}

impl<'a> WindowProcRequest<'a> {
    pub fn route_paint<F>(&mut self, f: F) -> &mut Self
    where
        F: for<'r> FnOnce(&'r ForeignWindow) -> Result<()>,
    {
        use winapi::um::winuser::WM_PAINT;
        if self.args.msg == WM_PAINT {
            if let Some(response) = self.response.take() {
                if let Some(window) = AnyWindow::new_from_attached(self.hwnd) {
                    if (f)(&window).is_ok() {
                        *response = WindowProcResponse::Done(0);
                    }
                } else {
                    warn!(target: "apiw", "Received message without window target for event: {}",
                          "route_paint");
                }
            } else {
                warn!(target: "apiw", "Duplicate route for event: {}",
                    "route_paint");
            }
        }
        self
    }

    pub fn route_close<F>(&mut self, f: F) -> &mut Self
    where
        F: for<'r> FnOnce(&'r ForeignWindow) -> Result<()>,
    {
        use winapi::um::winuser::WM_CLOSE;
        if self.args.msg == WM_CLOSE {
            if let Some(response) = self.response.take() {
                if let Some(window) = AnyWindow::new_from_attached(self.hwnd) {
                    if (f)(&window).is_ok() {
                        *response = WindowProcResponse::Done(0);
                    }
                } else {
                    warn!(target: "apiw", "Received message without window target for event: {}",
                          "route_close");
                }
            } else {
                warn!(target: "apiw", "Duplicate route for event: {}",
                      "route_close");
            }
        }
        self
    }

    pub fn route_mouse<F>(&mut self, f: F) -> &mut Self
    where
        F: for<'r, 's> FnOnce(&'r ForeignWindow, MouseEventArgs<'s>) -> Result<bool>,
    {
        use winapi::um::winuser::{WM_MOUSEFIRST, WM_MOUSELAST};
        if self.args.msg >= WM_MOUSEFIRST && self.args.msg <= WM_MOUSELAST {
            if let Some(response) = self.response.take() {
                if let Some(window) = AnyWindow::new_from_attached(self.hwnd) {
                    let mouse_args = MouseEventArgs(&self.args);
                    if let Ok(true) = (f)(&window, mouse_args) {
                        *response = WindowProcResponse::Done(0);
                    }
                } else {
                    warn!(target: "apiw", "Received message without window target for event: {}",
                          "route_mouse");
                }
            } else {
                warn!(target: "apiw", "Duplicate route for event: {}",
                      "route_mouse");
            }
        }
        self
    }
}

#[macro_export]
macro_rules! window_proc {
    ($nest_proc:expr) => {{
        unsafe extern "system" fn translator(
            hwnd: $crate::full_windows_api::shared::windef::HWND,
            msg: $crate::full_windows_api::shared::minwindef::UINT,
            wparam: $crate::full_windows_api::shared::minwindef::WPARAM,
            lparam: $crate::full_windows_api::shared::minwindef::LPARAM,
        ) -> $crate::full_windows_api::shared::minwindef::LRESULT {
            let mut response = $crate::windows_subsystem::window::WindowProcResponse::Fallback;
            {
                let request = $crate::windows_subsystem::window::WindowProcRequest {
                    hwnd,
                    args: $crate::windows_subsystem::window::WindowProcRequestArgs {
                        msg,
                        wparam,
                        lparam,
                    },
                    response: Some(&mut response),
                };

                ($nest_proc)(request);
            }

            use $crate::windows_subsystem::window::WindowProcResponse;
            match response {
                WindowProcResponse::Done(r) => r,
                WindowProcResponse::Fallback => {
                    $crate::full_windows_api::um::winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
                }
            }
        }
        translator
    }};
}

use std::num::NonZeroUsize;
use std::time::Duration;

type TimerProcInner = unsafe extern "system" fn(_: HWND, _: UINT, _: UINT_PTR, _: DWORD);

impl<T: ManagedStrategy> AnyWindow<T> {
    pub fn set_timer_with_id(
        &self,
        id: NonZeroUsize,
        interval: Duration,
        timer_proc: TimerProcInner,
    ) -> Result<&Self> {
        use winapi::um::winuser::SetTimer;
        use winapi::um::winuser::USER_TIMER_MAXIMUM;
        unsafe {
            let mut interval = interval
                .as_secs()
                .saturating_mul(60)
                .saturating_add(interval.subsec_millis() as _);
            if interval > USER_TIMER_MAXIMUM as _ {
                interval = USER_TIMER_MAXIMUM as _;
            }
            if 0 == SetTimer(
                self.data_ref().raw_handle(),
                id.get(),
                interval as _,
                Some(timer_proc),
            ) {
                return last_error();
            }
        }
        Ok(self)
    }
}

pub struct TimerProcRequest {
    pub hwnd: HWND,
}

impl TimerProcRequest {
    pub fn window(&self) -> Option<ForeignWindow> {
        ForeignWindow::new_from_attached(self.hwnd)
    }
}

#[macro_export]
macro_rules! timer_proc {
    ($nest_proc:expr) => {{
        unsafe extern "system" fn translator(
            hwnd: $crate::full_windows_api::shared::windef::HWND,
            arg2: $crate::full_windows_api::shared::minwindef::UINT,
            arg3: $crate::full_windows_api::shared::basetsd::UINT_PTR,
            arg4: $crate::full_windows_api::shared::minwindef::DWORD,
        ) {
            {
                let request = $crate::windows_subsystem::window::TimerProcRequest { hwnd };

                ($nest_proc)(request);
            }
        }
        translator
    }};
}
