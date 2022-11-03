use std::{
    convert::Infallible,
    ffi::{c_void, CString},
    marker::PhantomData,
    mem::transmute,
    os::raw::{c_char, c_int},
};

use crate::{FlutterEngineContextResult, PhantomUnsend, PhantomUnsync};

pub type FlutterEngineContextError = Infallible;

pub struct FlutterEngineContext {
    _unsync: PhantomUnsync,
    _unsend: PhantomUnsend,
}

const RTLD_LAZY: c_int = 1;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub type FlView = *mut c_void;
pub type FlTextureRegistrar = *mut c_void;
pub type FlBinaryMessenger = *mut c_void;
type GetFlutterViewProc = unsafe extern "C" fn(i64) -> FlView;
type GetFlutterTextureRegistrarProc = unsafe extern "C" fn(i64) -> FlTextureRegistrar;
type GetFlutterBinaryMessengerProc = unsafe extern "C" fn(i64) -> FlBinaryMessenger;

impl FlutterEngineContext {
    pub fn new() -> Self {
        Self {
            _unsync: PhantomData,
            _unsend: PhantomData,
        }
    }

    fn get_proc(name: &str) -> *mut c_void {
        let dl = unsafe { dlopen(std::ptr::null_mut(), RTLD_LAZY) };
        let name = CString::new(name).unwrap();
        let proc = unsafe { dlsym(dl, name.as_ptr()) };
        proc
    }

    pub fn get_flutter_view(&self, handle: i64) -> FlutterEngineContextResult<FlView> {
        let proc = Self::get_proc("FlutterEngineContextGetFlutterView");
        let proc: GetFlutterViewProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }

    pub fn get_binary_messenger(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlBinaryMessenger> {
        let proc = Self::get_proc("FlutterEngineContextGetBinaryMessenger");
        let proc: GetFlutterBinaryMessengerProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }

    pub fn get_texture_registry(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<FlTextureRegistrar> {
        let proc = Self::get_proc("FlutterEngineContextGetTextureRegistrar");
        let proc: GetFlutterTextureRegistrarProc = unsafe { transmute(proc) };
        Ok(unsafe { proc(handle) })
    }
}
