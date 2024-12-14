import { defineConfig } from 'vitepress';
import { PluginImpl } from './node/plugin';
import { fileURLToPath, URL } from 'node:url';
import { docsRs, reference } from './shared/url';

const plugins = PluginImpl.load();

export default defineConfig({
  title: 'tauri-store',
  description: 'Persistent stores for Tauri',
  base: '/tauri-store/',
  lang: 'en-US',
  srcDir: 'src',
  srcExclude: ['**/examples/**'],
  cleanUrls: true,
  metaChunk: true,

  vite: {
    build: {
      emptyOutDir: true,
    },
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('theme', import.meta.url)),
      },
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
        items: docsRsItems(),
      },
      {
        text: 'Reference',
        items: referenceItems(),
      },
    ],

    sidebar: [
      { text: 'Getting started', link: '/guide/getting-started' },
      { text: 'Persisting state', link: '/guide/persisting-state' },
      { text: 'Synchronization', link: '/guide/synchronization' },
      { text: 'Accessing from Rust', link: '/guide/accessing-from-rust' },
    ],

    outline: {
      level: 2,
    },
  },
});

function docsRsItems() {
  return plugins.map((plugin) => ({
    text: plugin.name,
    link: docsRs(plugin),
  }));
}

function referenceItems() {
  return plugins.map((plugin) => ({
    text: plugin.name,
    link: reference(plugin),
  }));
}
