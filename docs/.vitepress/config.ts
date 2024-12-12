import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';
import { defineConfig } from 'vitepress';

export default defineConfig({
  base: '/tauri-store/',
  title: 'tauri-store',
  description: 'Persistent stores for Tauri',
  lang: 'en-US',
  srcDir: 'src',
  vite: {
    css: {
      postcss: {
        plugins: [tailwindcss(), autoprefixer()],
      },
    },
    build: {
      emptyOutDir: true,
    },
  },
  themeConfig: {
    socialLinks: [
      { icon: 'github', link: 'https://github.com/ferreira-tb/tauri-store' },
      { icon: 'discord', link: 'https://discord.gg/ARd7McmVNv' },
    ],
    nav: [
      {
        text: 'docs.rs',
        items: [
          {
            text: 'Pinia',
            link: 'https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/',
          },
        ],
      },
      {
        text: 'Reference',
        items: [
          {
            text: 'Pinia',
            link: 'https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/index.html',
          },
        ],
      },
    ],
    sidebar: [
      {
        text: 'Pinia',
        collapsed: false,
        items: [
          { text: 'Getting started', link: '/pinia/getting-started' },
          { text: 'Persisting state', link: '/pinia/persisting-state' },
          { text: 'Accessing from Rust', link: '/pinia/accessing-from-rust' },
          { text: 'Synchronization', link: '/pinia/synchronization' },
        ],
      },
    ],
  },
});
