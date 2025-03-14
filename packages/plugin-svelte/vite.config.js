import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  build: {
    emptyOutDir: true,
    minify: false,
    rollupOptions: {
      external: [/^@tauri-apps/, /^@tauri-store/, 'svelte', 'svelte/store'],
    },
  },
});
