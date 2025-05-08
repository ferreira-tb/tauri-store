import '../../assets/style.css';
import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { TauriPluginPinia } from '@tauri-store/pinia/src/index.js';

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  TauriPluginPinia({
    saveOnChange: true,
    hooks: { error: console.error },
  })
);

app.use(pinia).mount('#app');
