import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { TauriPluginPinia } from 'tauri-plugin-pinia/src/index.ts';

const pinia = createPinia();
pinia.use(TauriPluginPinia);

createApp(App).use(pinia).mount('#app');
