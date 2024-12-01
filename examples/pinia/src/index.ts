import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia/src/index.ts';

const pinia = createPinia();
pinia.use(createPlugin());

createApp(App).use(pinia).mount('#app');
