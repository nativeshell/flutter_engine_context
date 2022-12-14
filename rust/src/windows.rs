use std::{
    ffi::{c_void, CString},
    fmt::Display,
    mem::transmute,
};

use crate::FlutterEngineContextResult;

#[derive(Debug)]
pub enum Error {
    InvalidHandle,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidHandle => write!(f, "invalid engine handle"),
        }
    }
}

impl std::error::Error for Error {}

pub struct PlatformContext {}

#[allow(clippy::upper_case_acronyms)]
type LPCSTR = *const i8;
#[allow(clippy::upper_case_acronyms)]
type HINSTANCE = isize;
#[allow(clippy::upper_case_acronyms)]
type HMODULE = isize;
#[allow(clippy::upper_case_acronyms)]
type HWND = isize;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpmodulename: LPCSTR) -> HINSTANCE;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> *mut c_void;
}

pub(crate) type FlutterView = HWND;
pub(crate) type FlutterTextureRegistry = FlutterDesktopTextureRegistrarRef;
pub(crate) type FlutterBinaryMessenger = FlutterDesktopMessengerRef;

type FlutterDesktopTextureRegistrarRef = *mut c_void;
type FlutterDesktopMessengerRef = *mut c_void;

type GetFlutterViewProc = unsafe extern "C" fn(i64) -> isize;
type GetTextureRegistrarProc = unsafe extern "C" fn(i64) -> FlutterDesktopTextureRegistrarRef;
type GetMessengerProc = unsafe extern "C" fn(i64) -> FlutterDesktopMessengerRef;

impl PlatformContext {
    pub fn new() -> Self {
        Self {}
    }

    fn get_proc(name: &str) -> *mut c_void {
        let module_name = CString::new("flutter_engine_context_plugin.dll").unwrap();
        let module = unsafe { GetModuleHandleA(module_name.as_ptr()) };
        let proc_name = CString::new(name).unwrap();
        unsafe { GetProcAddress(module, proc_name.as_ptr()) }
    }

    pub fn get_flutter_view(&self, handle: i64) -> FlutterEngineContextResult<HWND> {
        let proc = Self::get_proc("FlutterEngineContextGetFlutterView");
        let proc: GetFlutterViewProc = unsafe { transmute(proc) };
        let view = unsafe { proc(handle) };
        if view == 0 {
            Err(Error::InvalidHandle)
        } else {
            Ok(view)
        }
    }

    pub fn get_texture_registry(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlutterDesktopTextureRegistrarRef> {
        let proc = Self::get_proc("FlutterEngineContextGetTextureRegistrar");
        let proc: GetTextureRegistrarProc = unsafe { transmute(proc) };
        let registry = unsafe { proc(handle) };
        if registry.is_null() {
            Err(Error::InvalidHandle)
        } else {
            Ok(registry)
        }
    }

    pub fn get_binary_messenger(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlutterDesktopMessengerRef> {
        let proc = Self::get_proc("FlutterEngineContextGetBinaryMessenger");
        let proc: GetMessengerProc = unsafe { transmute(proc) };
        let messenger = unsafe { proc(handle) };
        if messenger.is_null() {
            Err(Error::InvalidHandle)
        } else {
            Ok(messenger)
        }
    }
}
