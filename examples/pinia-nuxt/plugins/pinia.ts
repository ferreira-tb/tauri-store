import type { Pinia } from 'pinia';
import { TauriPluginPinia } from '@tauri-store/pinia/src/index.js';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(
    TauriPluginPinia({
      autoStart: true,
      saveOnChange: true,
      hooks: { error: console.error },
    })
  );
});
