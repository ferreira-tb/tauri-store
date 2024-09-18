import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { exit } from '@tauri-apps/plugin-process';
import * as plugin from 'tauri-plugin-pinia/src/index.ts';
import { createPlugin } from 'tauri-plugin-pinia/src/index.ts';

const pinia = createPinia();
pinia.use(createPlugin());

createApp(App).use(pinia).mount('#app');

window.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') void exit();
});

function invokeThenLog(command: string) {
  invoke(command)
    .then((result) => {
      console.log(result);
    })
    .catch((err) => {
      console.error(err);
    });
}

Object.defineProperty(window, '_', {
  configurable: false,
  enumerable: true,
  writable: false,
  value: {
    plugin,
    getCounter: () => invokeThenLog('get_counter'),
    tryGetCounter: () => invokeThenLog('try_get_counter'),
    tryStoreState: () => invokeThenLog('try_store_state'),
  },
});
