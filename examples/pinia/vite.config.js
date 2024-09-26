import { resolve } from 'node:path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  clearScreen: false,
  plugins: [vue()],
  server: {
    port: 1422,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    emptyOutDir: true,
    minify: false,
    target: 'esnext',
  },
});
