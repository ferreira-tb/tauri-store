import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { TauriPluginPinia } from 'tauri-plugin-pinia/src/index.ts';

const app = createApp(App);
const pinia = createPinia();
pinia.use(TauriPluginPinia());

app.use(pinia).mount('#app');
