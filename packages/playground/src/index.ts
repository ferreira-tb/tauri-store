import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { exit } from '@tauri-apps/plugin-process';
import { createPlugin } from 'tauri-plugin-pinia/src/index.ts';

const pinia = createPinia();
pinia.use(createPlugin());

createApp(App).use(pinia).mount('#app');

window.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') void exit();
});
