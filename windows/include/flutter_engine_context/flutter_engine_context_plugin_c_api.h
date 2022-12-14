#ifndef FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_C_API_H_
#define FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_C_API_H_

#include <flutter_plugin_registrar.h>
#include <stdint.h>

#ifdef FLUTTER_PLUGIN_IMPL
#define FLUTTER_PLUGIN_EXPORT __declspec(dllexport)
#else
#define FLUTTER_PLUGIN_EXPORT __declspec(dllimport)
#endif

#if defined(__cplusplus)
extern "C" {
#endif

FLUTTER_PLUGIN_EXPORT void FlutterEngineContextPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar);

FLUTTER_PLUGIN_EXPORT size_t
FlutterEngineContextGetFlutterView(int64_t engine_handle);

FLUTTER_PLUGIN_EXPORT FlutterDesktopTextureRegistrarRef
FlutterEngineContextGetTextureRegistrar(int64_t engine_handle);

FLUTTER_PLUGIN_EXPORT FlutterDesktopMessengerRef
FlutterEngineContextGetBinaryMessenger(int64_t engine_handle);

#if defined(__cplusplus)
} // extern "C"
#endif

#endif // FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_C_API_H_
