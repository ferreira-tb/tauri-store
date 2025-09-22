package com.plugin.__SNAKE_PLUGIN_TITLE__

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class __PASCAL_PLUGIN_TITLE__Plugin(private val activity: Activity): Plugin(activity) {

@Command
    fun getAppSandboxPath(invoke: Invoke) {
        val sandboxPath = activity.filesDir.absolutePath

        val ret = JSObject()
        ret.put("path", sandboxPath)
        invoke.resolve(ret)
    }

}
