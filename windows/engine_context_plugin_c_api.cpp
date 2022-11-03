#include "include/engine_context/engine_context_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "engine_context_plugin.h"

void EngineContextPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  engine_context::EngineContextPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
