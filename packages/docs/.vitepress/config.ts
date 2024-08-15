import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';
import { defineConfig } from 'vitepress';

export default defineConfig({
  base: '/tauri-plugin-pinia/',
  title: 'tauri-plugin-pinia',
  description: 'Persistent Pinia stores for Tauri',
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
    socialLinks: [{ icon: 'github', link: 'https://github.com/ferreira-tb/tauri-plugin-pinia' }],
    nav: [
      {
        text: 'Reference',
        link: 'https://tb.dev.br/tauri-plugin-pinia/reference/index.html',
      },
      {
        text: 'Docs.rs',
        link: 'https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/',
      },
      {
        text: 'Playground',
        link: 'https://github.com/ferreira-tb/tauri-plugin-pinia/tree/main/packages/playground',
      },
    ],
    sidebar: [
      { text: 'Introduction', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      {
        text: 'Examples',
        collapsed: false,
        items: [
          { text: 'Persisting state', link: '/examples/persisting-state' },
          { text: 'Accessing from Rust', link: '/examples/accessing-from-rust' },
        ],
      },
    ],
  },
});
