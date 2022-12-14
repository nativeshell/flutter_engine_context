#ifndef FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_H_
#define FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace engine_context {

size_t GetFlutterView(int64_t engine_handle);
FlutterDesktopTextureRegistrarRef GetTextureRegistrar(int64_t engine_handle);
FlutterDesktopMessengerRef GetBinaryMessenger(int64_t engine_handle);

class FlutterEngineContextPlugin : public flutter::Plugin {
public:
  static void
  RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar,
                        FlutterDesktopPluginRegistrarRef raw_registrar);

  FlutterEngineContextPlugin(int64_t engine_handle);

  virtual ~FlutterEngineContextPlugin();

  // Disallow copy and assign.
  FlutterEngineContextPlugin(const FlutterEngineContextPlugin &) = delete;
  FlutterEngineContextPlugin &
  operator=(const FlutterEngineContextPlugin &) = delete;

private:
  int64_t engine_handle_;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

} // namespace engine_context

#endif // FLUTTER_PLUGIN_ENGINE_CONTEXT_PLUGIN_H_
