use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{
    GetWindowLongW, SetWindowLongPtrW, SetWindowLongW, GWLP_WNDPROC, GWL_STYLE, WM_USER,
    WS_MAXIMIZEBOX, WS_THICKFRAME,
};

use crate::APP;

pub const MB_UPDATE: UINT = WM_USER + 1;
pub const MB_STARTUP: UINT = WM_USER + 2;
pub const MB_SYSTEM_STAT: UINT = WM_USER + 3;

pub fn mbDisableWinStyle() {
    let hwnd = APP.mb.try_read().unwrap().GetWindowHandle();
    let style = unsafe { GetWindowLongW(hwnd, GWL_STYLE) };
    let style = style & !(WS_MAXIMIZEBOX as i32) & !(WS_THICKFRAME as i32);
    unsafe {
        SetWindowLongW(hwnd, GWL_STYLE, style);
    }
}

pub fn mbSetWinLongPtr(proc: isize) {
    use std::mem::transmute;
    unsafe {
        let mut wnd_proc = APP.wndproc.write().unwrap();
        let hwnd = APP.mb.try_read().unwrap().GetWindowHandle();
        // same with https://github.com/linebender/druid/issues/9
        *wnd_proc = transmute(SetWindowLongPtrW(hwnd, GWLP_WNDPROC, proc as _) as *const ())
    }
}
