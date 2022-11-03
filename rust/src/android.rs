use std::{fmt::Display, marker::PhantomData};

use jni::sys::jint;

use crate::{PhantomUnsend, PhantomUnsync};

pub struct FlutterEngineContext {
    java_vm: jni::JavaVM,
    class_loader: jni::objects::GlobalRef,
    _unsync: PhantomUnsync,
    _unsend: PhantomUnsend,
}

#[derive(Debug)]
pub enum FlutterEngineContextError {
    InvalidId,
    JNIError(jni::errors::Error),
}

pub type FlutterEngineContextResult<T> = Result<T, FlutterEngineContextError>;

impl Display for FlutterEngineContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlutterEngineContextError::JNIError(e) => e.fmt(f),
            FlutterEngineContextError::InvalidId => write!(f, "invalid engine id"),
        }
    }
}

impl std::error::Error for FlutterEngineContextError {}

impl From<jni::errors::Error> for FlutterEngineContextError {
    fn from(err: jni::errors::Error) -> Self {
        FlutterEngineContextError::JNIError(err)
    }
}

impl FlutterEngineContext {
    pub fn new(
        env: &jni::JNIEnv,
        class_loader: jni::objects::JObject,
    ) -> FlutterEngineContextResult<Self> {
        let java_vm = env.get_java_vm()?;
        let class_loader = env.new_global_ref(class_loader)?;
        Ok(Self {
            java_vm,
            class_loader,
            _unsync: PhantomData,
            _unsend: PhantomData,
        })
    }

    fn get_plugin_class<'a>(
        &'a self,
        env: &jni::JNIEnv<'a>,
    ) -> FlutterEngineContextResult<jni::objects::JClass<'a>> {
        let plugin_class = env
            .call_method(
                self.class_loader.as_obj(),
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[env
                    .new_string(
                        "dev/nativeshell/flutter_engine_context/FlutterEngineContextPlugin",
                    )?
                    .into()],
            )?
            .l()?;
        Ok(plugin_class.into())
    }

    pub fn get_activity(&self, handle: i64) -> FlutterEngineContextResult<jni::objects::GlobalRef> {
        let id: jint = handle
            .try_into()
            .map_err(|_| FlutterEngineContextError::InvalidId)?;
        let env = self.java_vm.get_env()?;
        let class = self.get_plugin_class(&env)?;
        let view = env
            .call_static_method(
                class,
                "getActivity",
                "(I)Landroid/app/Activity;",
                &[id.into()],
            )?
            .l()?;
        Ok(env.new_global_ref(view)?)
    }

    pub fn get_flutter_view(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<jni::objects::GlobalRef> {
        let id: jint = handle
            .try_into()
            .map_err(|_| FlutterEngineContextError::InvalidId)?;
        let env = self.java_vm.get_env()?;
        let class = self.get_plugin_class(&env)?;
        let view = env
            .call_static_method(
                class,
                "getFlutterView",
                "(I)Lio/flutter/embedding/android/FlutterView;",
                &[id.into()],
            )?
            .l()?;
        Ok(env.new_global_ref(view)?)
    }

    pub fn get_binary_messenger(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<jni::objects::GlobalRef> {
        let id: jint = handle
            .try_into()
            .map_err(|_| FlutterEngineContextError::InvalidId)?;
        let env = self.java_vm.get_env()?;
        let class = self.get_plugin_class(&env)?;
        let view = env
            .call_static_method(
                class,
                "getBinaryMessenger",
                "(I)Lio/flutter/plugin/common/BinaryMessenger;",
                &[id.into()],
            )?
            .l()?;
        Ok(env.new_global_ref(view)?)
    }

    pub fn get_texture_registry(
        &self,
        handle: i64,
    ) -> FlutterEngineContextResult<jni::objects::GlobalRef> {
        let id: jint = handle
            .try_into()
            .map_err(|_| FlutterEngineContextError::InvalidId)?;
        let env = self.java_vm.get_env()?;
        let class = self.get_plugin_class(&env)?;
        let view = env
            .call_static_method(
                class,
                "getTextureRegistry",
                "(I)Lio/flutter/view/TextureRegistry;",
                &[id.into()],
            )?
            .l()?;
        Ok(env.new_global_ref(view)?)
    }
}
