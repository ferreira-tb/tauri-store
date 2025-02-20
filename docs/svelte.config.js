import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      pages: 'dist',
      assets: 'dist',
      precompress: false,
      strict: true,
    }),
    paths: {
      base: '/tauri-store',
    },
    alias: {
      $components: './src/components',
      $content: './src/content',
      $layout: './src/layout',
      $stores: './src/lib/stores',
    },
  },
};

export default config;
