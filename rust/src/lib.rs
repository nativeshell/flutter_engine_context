#![allow(clippy::new_without_default)]

use std::{marker::PhantomData, cell::Cell, sync::MutexGuard};

#[cfg(target_os = "android")]
#[path = "android.rs"]
pub mod platform;

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[path = "darwin.rs"]
pub mod platform;

pub type FlutterEngineContextError = platform::FlutterEngineContextError;
pub type FlutterEngineContextResult<T> = platform::FlutterEngineContextResult<T>;
pub type FlutterEngineContext = platform::FlutterEngineContext;

pub(crate) type PhantomUnsync = PhantomData<Cell<()>>;
pub(crate) type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
