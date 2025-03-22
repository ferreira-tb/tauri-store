import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { onError } from './commands';
import {
  clearAutosave,
  getDefaultSaveStrategy,
  getSaveStrategy,
  getStoreCollectionPath,
  getStoreIds,
  getStorePath,
  getStoreState,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
  setStoreCollectionPath,
  setStoreOptions,
  TauriPluginPinia,
} from '@tauri-store/pinia/src/index.js';

declare global {
  interface Window {
    $plugin: any;
    $store: any;
  }
}

const $plugin = {
  clearAutosave,
  getDefaultSaveStrategy,
  getSaveStrategy,
  getStoreCollectionPath,
  getStoreIds,
  getStorePath,
  getStoreState,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
  setStoreCollectionPath,
  setStoreOptions,
};

if (!Object.hasOwn(window, '$plugin')) {
  Object.defineProperty(window, '$plugin', {
    value: $plugin,
    enumerable: true,
    configurable: false,
    writable: false,
  });
}

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  TauriPluginPinia({
    hooks: { error: onError },
  })
);

app.use(pinia).mount('#app');
