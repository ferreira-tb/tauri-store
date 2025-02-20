import type { Pinia } from 'pinia';
import { onError } from '../utils/commands';
import { TauriPluginPinia } from 'tauri-plugin-pinia/src/index.js';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(TauriPluginPinia({ onError }));
});
