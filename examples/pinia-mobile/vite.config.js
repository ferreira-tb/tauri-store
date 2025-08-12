import { env } from 'node:process';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  clearScreen: false,
  plugins: [vue()],
  server: {
    port: 1422,
    strictPort: true,
    host: env.TAURI_DEV_HOST || false,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    minify: false,
    target: 'esnext',
  },
});
