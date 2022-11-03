#![allow(clippy::new_without_default)]

use std::{cell::Cell, marker::PhantomData, sync::MutexGuard};

#[cfg(target_os = "android")]
#[path = "android.rs"]
pub mod platform;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod platform;

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[path = "darwin.rs"]
pub mod platform;

pub type FlutterEngineContextError = platform::FlutterEngineContextError;
pub type FlutterEngineContextResult<T> = Result<T, FlutterEngineContextError>;

pub type FlutterEngineContext = platform::FlutterEngineContext;

pub(crate) type PhantomUnsync = PhantomData<Cell<()>>;
pub(crate) type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
