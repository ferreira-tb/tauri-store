import tailwind from 'tailwindcss';
import { defineConfig } from 'vite';
import autoprefixer from 'autoprefixer';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },
  build: {
    emptyOutDir: true,
    minify: true,
  },
});
