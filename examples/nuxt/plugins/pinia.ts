import type { Pinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(createPlugin());
});