import SwiftRs
import Tauri
import UIKit
import WebKit

class __PASCAL_PLUGIN_TITLE__Plugin: Plugin {
  @objc public func getAppSandboxPath(_ invoke: Invoke) throws {
    if let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
      invoke.resolve(["path": documentsPath.path])
    } else {
      invoke.reject("Unable to get sandbox path")
    }
  }

}

@_cdecl("init_plugin___SNAKE_PLUGIN_TITLE__")
func initPlugin() -> Plugin {
  return __PASCAL_PLUGIN_TITLE__Plugin()
}
