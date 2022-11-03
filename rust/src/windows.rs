use std::{
    convert::Infallible,
    ffi::{c_void, CString},
    marker::PhantomData,
    mem::transmute,
};

use crate::{FlutterEngineContextResult, PhantomUnsend, PhantomUnsync};

pub type FlutterEngineContextError = Infallible;

pub struct FlutterEngineContext {
    _unsync: PhantomUnsync,
    _unsend: PhantomUnsend,
}

pub type LPCSTR = *const i8;
pub type HINSTANCE = isize;
pub type HMODULE = isize;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpmodulename: LPCSTR) -> HINSTANCE;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> *mut c_void;
}

pub type FlutterDesktopTextureRegistrarRef = *mut c_void;
pub type FlutterDesktopMessengerRef = *mut c_void;

type GetFlutterViewProc = unsafe extern "C" fn(i64) -> isize;
type GetTextureRegistrarProc = unsafe extern "C" fn(i64) -> FlutterDesktopTextureRegistrarRef;
type GetMessengerProc = unsafe extern "C" fn(i64) -> FlutterDesktopMessengerRef;

impl FlutterEngineContext {
    pub fn new() -> Self {
        Self {
            _unsync: PhantomData,
            _unsend: PhantomData,
        }
    }

    fn get_proc(name: &str) -> *mut c_void {
        let module_name = CString::new("flutter_engine_context_plugin.dll").unwrap();
        let module = unsafe { GetModuleHandleA(module_name.as_ptr()) };
        let proc_name = CString::new(name).unwrap();
        let proc = unsafe { GetProcAddress(module, proc_name.as_ptr()) };
        proc
    }

    pub fn get_flutter_view(&self, handle: i64) -> FlutterEngineContextResult<isize> {
        let proc = Self::get_proc("FlutterEngineContextGetFlutterView");
        let proc: GetFlutterViewProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }

    pub fn get_texture_registry(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlutterDesktopTextureRegistrarRef> {
        let proc = Self::get_proc("FlutterEngineContextGetTextureRegistrar");
        let proc: GetTextureRegistrarProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }

    pub fn get_binary_messenger(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlutterDesktopMessengerRef> {
        let proc = Self::get_proc("FlutterEngineContextGetBinaryMessenger");
        let proc: GetMessengerProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }
}
