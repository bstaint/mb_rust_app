#![allow(non_snake_case)]
use lazy_static::lazy_static;
use miniblink::interface::Type::MB;
use std::error::Error;
use std::sync::RwLock;
use winapi::shared::windef::HWND;
use winapi::um::winuser::WNDPROC;

pub mod bind;
pub mod fnc;
pub mod message;
pub mod system;
pub mod win32;

pub type Result = std::result::Result<(), Box<dyn Error>>;

lazy_static! {
    pub static ref APP: AppData = AppData::new();
}

pub struct AppData {
    pub wndproc: RwLock<WNDPROC>,
    pub mb: RwLock<MB>,
    pub hwnd: RwLock<HWND>,
}

impl AppData {
    fn new() -> Self {
        Self {
            mb: RwLock::new(MB::new()),
            wndproc: RwLock::new(None),
            hwnd: RwLock::new(std::ptr::null_mut()),
        }
    }
}

unsafe impl Send for AppData {}
unsafe impl Sync for AppData {}
