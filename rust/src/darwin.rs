use std::{convert::Infallible, marker::PhantomData};

use cocoa::base::id;
use objc::{class, msg_send, sel, sel_impl};

use crate::PhantomUnsync;

pub type FlutterEngineContextError = Infallible;

pub type FlutterEngineContextResult<T> = Result<T, FlutterEngineContextError>;

pub struct FlutterEngineContext {
    _unsync: PhantomUnsync,
    _unsend: PhantomUnsync,
}

impl FlutterEngineContext {
    pub fn new() -> Self {
        Self {
            _unsync: PhantomData,
            _unsend: PhantomData,
        }
    }

    pub fn get_flutter_view(&self, handle: i64) -> FlutterEngineContextResult<id> {
        unsafe {
            let view: id = msg_send![class!(FlutterEngineContextPlugin), getFlutterView: handle];
            Ok(view)
        }
    }

    pub fn get_texture_registry(&self, handle: i64) -> FlutterEngineContextResult<id> {
        unsafe {
            let view: id = msg_send![
                class!(FlutterEngineContextPlugin),
                getTextureRegistry: handle
            ];
            Ok(view)
        }
    }

    pub fn get_binary_messenger(&self, handle: i64) -> FlutterEngineContextResult<id> {
        unsafe {
            let view: id = msg_send![
                class!(FlutterEngineContextPlugin),
                getBinaryMessenger: handle
            ];
            Ok(view)
        }
    }
}
