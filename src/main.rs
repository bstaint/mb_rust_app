use log::debug;
use miniblink::interface::Type::*;
use miniblink_test::{bind::*, fnc::*, message::*, win32::*, Result, APP};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;

fn main() -> Result {
    simple_log::quick()?;

    MB::Initialize();
    MB::EnableHighDPISupport();
    MB::JsBindFunction("update_website", js_update_website, 0);
    MB::JsBindFunction("system_stat", js_system_stat, 0);

    // 测试自定义miniblink API
    debug!("mbVersion: {}", mbVersion());

    APP.mb
        .write()?
        .SetDebugConfig("antiAlias", "1")
        .SetDebugConfig("wakeMinInterval", "6")
        .CreateWebWindow(Window {
            width: 600,
            height: 400,
            ..Default::default()
        })
        .LoadFile("./template.html")
        .OnWindowDestroy(quit)
        .OnDocumentReady(startup)
        // .OnLoadUrlBegin(|webView: Webview, params: i32, url: *const i8, job: Netjob| {
        //     job.HookRequest();
        // }, 0)
        // .OnLoadUrlEnd(|webView: Webview, params: i32, url: *const i8, job: Netjob, buf: *const i8, len: i32| {
        // }, 0)
        .SetWindowTitle("Miniblink Demo")
        .MoveToCenter();

    *APP.hwnd.write()? = APP.mb.try_read()?.GetWindowHandle();
    // 禁用Windows窗口 && 设置消息处理方法
    mbDisableWinStyle();
    mbSetWinLongPtr(mb_window_proc as isize);

    MB::RunMessageLoop();
    Ok(())
}

unsafe extern "system" fn mb_window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        MB_UPDATE => on_update(lparam),
        MB_SYSTEM_STAT => on_system_stat(lparam),
        MB_STARTUP => on_startup(),
        _ => on_window_proc(hwnd, msg, wparam, lparam),
    }
}
