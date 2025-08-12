package com.plugin.tauri.store

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke



@TauriPlugin
class StorePlugin(private val activity: Activity): Plugin(activity) {


   @Command
    fun getAppSandboxPath(invoke: Invoke) {
        // Récupère le chemin vers le répertoire "files" de l'application
        val sandboxPath = activity.filesDir.absolutePath

        val ret = JSObject()
        ret.put("path", sandboxPath)
        invoke.resolve(ret)
    }
}
