#import "EngineContextPlugin.h"

// Flutter API doesn't provide an official way to get a view from registrar.
// This will likely break in future when multiple views per engine are
// supported. But that will be a major breaking change anyway.
@interface _FlutterPluginRegistrar : NSObject
@property(readwrite, nonatomic) FlutterEngine *flutterEngine;
@end

@interface _FlutterEngineContext : NSObject {
@public
  __weak FlutterEngine *engine;
}

@end

@implementation _FlutterEngineContext

@end

@interface FlutterEngineContextPlugin () {
  int64_t engineHandle;
}
@end

@implementation FlutterEngineContextPlugin

static NSMutableDictionary *registry;
static int64_t nextHandle = 1;

+ (void)initialize {
  registry = [NSMutableDictionary new];
}

+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar> *)registrar {
  FlutterEngineContextPlugin *instance =
      [[FlutterEngineContextPlugin alloc] init];
  instance->engineHandle = nextHandle;
  ++nextHandle;

  _FlutterEngineContext *context = [_FlutterEngineContext new];
  context->engine = ((_FlutterPluginRegistrar *)registrar).flutterEngine;
  // There is no unregister callback on macOS, which means we'll leak
  // an _FlutterEngineContext instance for every engine. Fortunately the
  // instance is tiny and only uses weak pointers to reference engine artifacts.
  [registry setObject:context forKey:@(instance->engineHandle)];

  FlutterMethodChannel *channel = [FlutterMethodChannel
      methodChannelWithName:@"dev.nativeshell.flutter_engine_context"
            binaryMessenger:[registrar messenger]];
  [registrar addMethodCallDelegate:instance channel:channel];
}

- (void)handleMethodCall:(FlutterMethodCall *)call
                  result:(FlutterResult)result {
  if ([@"getEngineHandle" isEqualToString:call.method]) {
    result(@(engineHandle));
  } else {
    result(FlutterMethodNotImplemented);
  }
}

+ (UIView *)getFlutterView:(int64_t)engineHandle {
  _FlutterEngineContext *context = [registry objectForKey:@(engineHandle)];
  return context->engine.viewController.view;
}

+ (id<FlutterTextureRegistry>)getTextureRegistry:(int64_t)engineHandle {
  _FlutterEngineContext *context = [registry objectForKey:@(engineHandle)];
  return context->engine;
}

+ (id<FlutterBinaryMessenger>)getBinaryMessenger:(int64_t)engineHandle {
  _FlutterEngineContext *context = [registry objectForKey:@(engineHandle)];
  return context->engine.binaryMessenger;
}

@end
