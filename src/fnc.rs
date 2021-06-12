use libloading::Symbol;
use miniblink::interface::Type::MB;
use miniblink::mb::util::cToRustStr;

type MbVersion = fn() -> usize;
type MbVersionString = fn() -> *const i8;

pub fn mbVersion() -> usize {
    let wkeVersion: Symbol<MbVersion> = unsafe { MB::GetNodeDll().get(b"wkeVersion").unwrap() };

    wkeVersion()
}

pub fn mbVersionString<'a>() -> &'a str {
    let wkeVersionString: Symbol<MbVersionString> =
        unsafe { MB::GetNodeDll().get(b"wkeVersionString").unwrap() };

    cToRustStr(wkeVersionString())
}
