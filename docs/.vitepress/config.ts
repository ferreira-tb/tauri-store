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
    socialLinks: [{ icon: 'github', link: 'https://github.com/ferreira-tb/tauri-plugin-pinia' }],
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
