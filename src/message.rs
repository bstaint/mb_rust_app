use crate::APP;
use log::debug;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::CallWindowProcW;

#[inline]
pub fn on_window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let wnd_proc = APP.wndproc.read().unwrap();
    unsafe { CallWindowProcW(*wnd_proc, hwnd, msg, wparam, lparam) }
}

pub fn on_startup() -> LRESULT {
    debug!("MB_STARTUP");
    APP.mb.write().unwrap().ShowWindow();
    0
}

pub fn on_system_stat(param: LPARAM) -> LRESULT {
    let json: Box<String> = unsafe { Box::from_raw(param as *mut _) };
    // debug!("MB_SYSTEM_STAT lparam: {}", json);
    APP.mb
        .write()
        .unwrap()
        .RunJS(&format!("vm.system_stat = {}", json));
    0
}

pub fn on_update(param: LPARAM) -> LRESULT {
    let _json: Box<String> = unsafe { Box::from_raw(param as *mut _) };
    // debug!("MB_THREAD_MSG lparam: {}", json);
    APP.mb.write().unwrap().RunJS(&format!(
        r#"
        vm.version = "{}";
        vm.$indicator.close();
    "#,
        "1.0.0"
    ));
    0
}
