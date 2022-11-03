import 'package:flutter/services.dart';

class FlutterEngineContext {
  static final instance = FlutterEngineContext();

  final methodChannel = const MethodChannel('dev.nativeshell.engine_context');

  static int? _engineHandle;
  Future<int> getEngineHandle() async {
    _engineHandle ??= await methodChannel.invokeMethod<int>('getEngineHandle');
    return _engineHandle!;
  }
}
