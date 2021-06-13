use crate::system::system_stat;
use crate::win32::{MB_STARTUP, MB_SYSTEM_STAT, MB_UPDATE};
use crate::APP;
use miniblink::interface::Type::{jsExecState, jsValue, MB};
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::{PostMessageW, PostQuitMessage};

pub fn mb_startup() {
    use std::sync::Once;
    static STARTUP: Once = Once::new();
    STARTUP.call_once(|| {
        let hwnd = APP.hwnd.read().unwrap();
        unsafe {
            PostMessageW(*hwnd, MB_STARTUP, 0, 0);
        };
    });
}

#[inline]
pub fn mb_quit() {
    unsafe { PostQuitMessage(0) }
}

pub fn js_update_website(_es: jsExecState) -> jsValue {
    use std::time::Duration;

    std::thread::spawn(|| {
        // 实际业务逻辑
        std::thread::sleep(Duration::from_secs(3));
        let hwnd = APP.hwnd.read().unwrap();
        let param = Box::new("{}".to_string());
        unsafe {
            PostMessageW(*hwnd, MB_UPDATE, 0, Box::into_raw(param) as LPARAM);
        };
    });
    MB::jsUndefined()
}

pub fn js_system_stat(_es: jsExecState) -> jsValue {
    std::thread::spawn(|| {
        let stat = system_stat();
        let hwnd = APP.hwnd.read().unwrap();
        let param = Box::new(serde_json::to_string(&stat).unwrap_or_default());
        unsafe {
            PostMessageW(*hwnd, MB_SYSTEM_STAT, 0, Box::into_raw(param) as LPARAM);
        };
    });
    MB::jsUndefined()
}
