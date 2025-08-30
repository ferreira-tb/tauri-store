import SwiftRs
import Tauri
import UIKit
import WebKit

class StorePlugin: Plugin {
  @objc public func getSandboxPath(_ invoke: Invoke) throws {
    if let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
      invoke.resolve(["path": documentsPath.path])
    } else {
      invoke.reject("Unable to get sandbox path")
    }
  }

}

@_cdecl("init_plugin_tauri_store")
func initPlugin() -> Plugin {
  return StorePlugin()
}
