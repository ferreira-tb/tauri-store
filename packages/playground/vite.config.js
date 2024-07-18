import { resolve } from 'node:path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { existsSync } from 'node:fs';

export default defineConfig({
  clearScreen: false,
  plugins: [vue()],
  server: {
    port: 1422,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**']
    }
  },
  build: {
    emptyOutDir: true,
    minify: false,
    target: 'esnext',
    rollupOptions: {
      input: entries()
    }
  }
});

function entries() {
  return [1, 2, 3, 4].reduce((acc, i) => {
    acc[`window-${i}`] = resolve(__dirname, `src/windows/window-${i}/index.html`);
    return acc;
  }, {});
}
