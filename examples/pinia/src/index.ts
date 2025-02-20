import '../../assets/style.css';
import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { onError } from './commands';
import { TauriPluginPinia } from 'tauri-plugin-pinia/src/index.js';

const app = createApp(App);
const pinia = createPinia();
pinia.use(TauriPluginPinia({ onError }));

app.use(pinia).mount('#app');
