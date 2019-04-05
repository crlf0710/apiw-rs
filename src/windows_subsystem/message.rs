use std::mem::uninitialized;
use std::ptr::null_mut;
use derive_more::From;
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::{BOOL, DWORD, UINT};
use winapi::shared::windef::HWND;
use winapi::um::winnt::LONG;
use winapi::um::winuser::MSG;
use wio::error::last_error;
use wio::Result;

use crate::shared::booleanize;

/// ECMA-234 Clause 6 CallWindowProc
fn call_window_proc() {
    unimplemented!()
}

#[derive(From)]
pub struct Message(MSG);

#[derive(From)]
pub struct MessageResult(LRESULT);

impl Message {
    /// ECMA-234 Clause 7 DispatchMessage
    pub fn dispatch(self) -> MessageResult {
        use winapi::um::winuser::DispatchMessageW;
        unsafe { DispatchMessageW(&self.0).into() }
    }
}

pub enum QuitOrNormalMsg {
    QuitMsg,
    NormalMsg(Message),
}

impl QuitOrNormalMsg {
    pub fn not_quit(self) -> Option<Message> {
        match self {
            QuitOrNormalMsg::QuitMsg => None,
            QuitOrNormalMsg::NormalMsg(m) => Some(m),
        }
    }
}

pub struct MessageSimpleFilter {
    hwnd: HWND,
    min: UINT,
    max: UINT,
}

impl MessageSimpleFilter {
    pub const fn new() -> Self {
        MessageSimpleFilter {
            hwnd: null_mut(),
            min: 0,
            max: 0,
        }
    }
}

pub struct MessageFilter {
    hwnd: HWND,
    min: UINT,
    max: UINT,
    kinds: UINT,
}

impl MessageFilter {
    pub fn new() -> Self {
        MessageFilter {
            hwnd: null_mut(),
            min: 0,
            max: 0,
            kinds: 0,
        }
    }
}

#[derive(From)]
pub struct MessagePos(DWORD);

#[derive(From)]
pub struct MessageTime(LONG);

pub struct ForeignMessageLoop;

impl ForeignMessageLoop {
    pub fn for_current_thread() -> Self {
        ForeignMessageLoop
    }

    /// ECMA-234 Clause 8 GetMessage
    pub fn poll_wait(&mut self) -> Result<QuitOrNormalMsg> {
        const FILTER: MessageSimpleFilter = MessageSimpleFilter::new();
        self.poll_wait_with_filter(&FILTER)
    }

    /// ECMA-234 Clause 8 GetMessage
    pub fn poll_wait_with_filter(
        &mut self,
        filter: &MessageSimpleFilter,
    ) -> Result<QuitOrNormalMsg> {
        use winapi::um::winuser::GetMessageW;
        unsafe {
            let mut msg: MSG = uninitialized();
            let ret: BOOL = GetMessageW(&mut msg, filter.hwnd, filter.min, filter.max);
            if ret == -1 {
                last_error()
            } else if ret == 0 {
                Ok(QuitOrNormalMsg::QuitMsg)
            } else {
                Ok(QuitOrNormalMsg::NormalMsg(msg.into()))
            }
        }
    }

    // internal
    fn peek_internal(&mut self, filter: &MessageFilter, extra_flags: UINT) -> Option<Message> {
        use winapi::um::winuser::PeekMessageW;
        unsafe {
            let mut msg: MSG = uninitialized();
            let ret: BOOL = PeekMessageW(
                &mut msg,
                filter.hwnd,
                filter.min,
                filter.max,
                filter.kinds | extra_flags,
            );
            if booleanize(ret) {
                Some(msg.into())
            } else {
                None
            }
        }
    }

    /// ECMA-234 Clause 8 PeekMessage
    pub fn peek_with_filter(&mut self, filter: &MessageFilter) -> Option<Message> {
        use winapi::um::winuser::PM_NOREMOVE;
        self.peek_internal(filter, PM_NOREMOVE)
    }

    /// ECMA-234 Clause 8 PeekMessage
    pub fn peek_and_consume_message_with_filter(
        &mut self,
        filter: &MessageFilter,
    ) -> Option<Message> {
        use winapi::um::winuser::PM_REMOVE;
        self.peek_internal(filter, PM_REMOVE)
    }

    /// ECMA-234 Clause 8 PeekMessage
    pub fn peek_with_filter_no_yield(&mut self, filter: &MessageFilter) -> Option<Message> {
        use winapi::um::winuser::{PM_NOREMOVE, PM_NOYIELD};
        self.peek_internal(filter, PM_NOREMOVE | PM_NOYIELD)
    }

    /// ECMA-234 Clause 8 PeekMessage
    pub fn peek_and_consume_message_with_filter_no_yield(
        &mut self,
        filter: &MessageFilter,
    ) -> Option<Message> {
        use winapi::um::winuser::{PM_NOYIELD, PM_REMOVE};
        self.peek_internal(filter, PM_REMOVE | PM_NOYIELD)
    }

    /// ECMA-234 Clause 9 WaitMessage
    pub fn wait_for_next_incoming(&mut self) -> Result<()> {
        use winapi::um::winuser::WaitMessage;
        unsafe {
            if booleanize(WaitMessage()) {
                Ok(())
            } else {
                last_error()
            }
        }
    }

    /// ECMA-234 Clause 10 GetMessagePos
    pub fn last_message_pos(&mut self) -> MessagePos {
        use winapi::um::winuser::GetMessagePos;
        unsafe { GetMessagePos().into() }
    }

    /// ECMA-234 Clause 10 GetMessageTime
    pub fn last_message_time(&mut self) -> MessageTime {
        use winapi::um::winuser::GetMessageTime;
        unsafe { GetMessageTime().into() }
    }
}


