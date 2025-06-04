import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'tauri-store',
  description: 'Persistent stores for Tauri',
  base: '/tauri-store/',
  srcDir: 'src',
  cleanUrls: true,
  metaChunk: true,
  lastUpdated: true,
  sitemap: {
    hostname: 'https://tb.dev.br/tauri-store/',
  },

  vite: {
    build: {
      emptyOutDir: true,
      minify: true,
      target: 'es2015',
    },
  },

  markdown: {
    theme: {
      light: 'vitesse-light',
      dark: 'vitesse-dark',
    },
  },

  head: [['link', { rel: 'icon', href: '/tauri-store/favicon.ico' }]],

  themeConfig: {
    search: {
      provider: 'local',
    },
    outline: {
      level: [2, 3],
    },

    nav: [
      { text: 'Guide', link: '/' },
      {
        text: 'JavaScript',
        items: [
          {
            text: 'tauri-store',
            link: 'https://tb.dev.br/tauri-store/js-docs/tauri-store',
          },
          {
            text: '@tauri-store/pinia',
            link: 'https://tb.dev.br/tauri-store/js-docs/plugin-pinia',
          },
          {
            text: '@tauri-store/svelte',
            link: 'https://tb.dev.br/tauri-store/js-docs/plugin-svelte',
          },
          {
            text: '@tauri-store/valtio',
            link: 'https://tb.dev.br/tauri-store/js-docs/plugin-valtio',
          },
          {
            text: '@tauri-store/vue',
            link: 'https://tb.dev.br/tauri-store/js-docs/plugin-vue',
          },
          {
            text: '@tauri-store/zustand',
            link: 'https://tb.dev.br/tauri-store/js-docs/plugin-zustand',
          },
        ],
      },
      {
        text: 'Rust',
        items: [
          {
            text: 'tauri-store',
            link: 'https://docs.rs/tauri-store/latest/tauri_store',
          },
          {
            text: 'tauri-plugin-pinia',
            link: 'https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia',
          },
          {
            text: 'tauri-plugin-svelte',
            link: 'https://docs.rs/tauri-plugin-svelte/latest/tauri_plugin_svelte',
          },
          {
            text: 'tauri-plugin-valtio',
            link: 'https://docs.rs/tauri-plugin-valtio/latest/tauri_plugin_valtio',
          },
          {
            text: 'tauri-plugin-vue',
            link: 'https://docs.rs/tauri-plugin-vue/latest/tauri_plugin_vue',
          },
          {
            text: 'tauri-plugin-zustand',
            link: 'https://docs.rs/tauri-plugin-zustand/latest/tauri_plugin_zustand',
          },
        ],
      },
      {
        text: 'Changelog',
        items: [
          {
            text: 'tauri-store',
            link: '/changelog',
          },
          {
            text: '@tauri-store/pinia',
            link: '/plugin-pinia/changelog',
          },
          {
            text: '@tauri-store/svelte',
            link: '/plugin-svelte/changelog',
          },
          {
            text: '@tauri-store/valtio',
            link: '/plugin-valtio/changelog',
          },
          {
            text: '@tauri-store/vue',
            link: '/plugin-vue/changelog',
          },
          {
            text: '@tauri-store/zustand',
            link: '/plugin-zustand/changelog',
          },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/ferreira-tb/tauri-store' },
      { icon: 'discord', link: 'https://discord.gg/ARd7McmVNv' },
    ],

    sidebar: {
      '/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/guide/getting-started' },
            { text: 'Persisting state', link: '/guide/persisting-state' },
            { text: 'Synchronization', link: '/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/guide/lifecycle-hooks' },
          ],
        },
      ],
      '/plugin-pinia/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/plugin-pinia/guide/getting-started' },
            { text: 'Persisting state', link: '/plugin-pinia/guide/persisting-state' },
            { text: 'Synchronization', link: '/plugin-pinia/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/plugin-pinia/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/plugin-pinia/guide/lifecycle-hooks' },
          ],
        },
      ],
      '/plugin-svelte/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/plugin-svelte/guide/getting-started' },
            { text: 'Persisting state', link: '/plugin-svelte/guide/persisting-state' },
            { text: 'Synchronization', link: '/plugin-svelte/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/plugin-svelte/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/plugin-svelte/guide/lifecycle-hooks' },
          ],
        },
      ],
      '/plugin-valtio/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/plugin-valtio/guide/getting-started' },
            { text: 'Persisting state', link: '/plugin-valtio/guide/persisting-state' },
            { text: 'Synchronization', link: '/plugin-valtio/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/plugin-valtio/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/plugin-valtio/guide/lifecycle-hooks' },
          ],
        },
      ],
      '/plugin-vue/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/plugin-vue/guide/getting-started' },
            { text: 'Persisting state', link: '/plugin-vue/guide/persisting-state' },
            { text: 'Synchronization', link: '/plugin-vue/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/plugin-vue/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/plugin-vue/guide/lifecycle-hooks' },
          ],
        },
      ],
      '/plugin-zustand/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting started', link: '/plugin-zustand/guide/getting-started' },
            { text: 'Persisting state', link: '/plugin-zustand/guide/persisting-state' },
            { text: 'Synchronization', link: '/plugin-zustand/guide/synchronization' },
            { text: 'Accessing from Rust', link: '/plugin-zustand/guide/accessing-from-rust' },
            { text: 'Lifecycle hooks', link: '/plugin-zustand/guide/lifecycle-hooks' },
          ],
        },
      ],
    },
  },
});
