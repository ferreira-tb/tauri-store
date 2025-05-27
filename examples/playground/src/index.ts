import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import * as $vue from '@tauri-store/vue/src/index.js';
import * as $pinia from '@tauri-store/pinia/src/index.js';
import * as $valtio from '@tauri-store/valtio/src/index.js';
import * as $zustand from '@tauri-store/zustand/src/index.js';
import * as $svelte from '@tauri-store/svelte/src/lib/index.js';

declare global {
  interface Window {
    $pinia: typeof $pinia;
    $svelte: typeof $svelte;
    $valtio: typeof $valtio;
    $vue: typeof $vue;
    $zustand: typeof $zustand;
  }
}

window.$pinia = $pinia;
window.$svelte = $svelte;
window.$valtio = $valtio;
window.$vue = $vue;
window.$zustand = $zustand;

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  $pinia.TauriPluginPinia({
    hooks: { error: console.error },
  })
);

app.use(pinia).mount('#app');
