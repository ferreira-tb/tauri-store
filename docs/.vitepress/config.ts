import { defineConfig } from 'vitepress';
import { PluginImpl } from './node/plugin';
import { fileURLToPath, URL } from 'node:url';
import { docsRs, reference } from './shared/url';

type ThemeConfig = NonNullable<Parameters<typeof defineConfig>[0]['themeConfig']>;

const plugins = PluginImpl.load();

const nav: ThemeConfig['nav'] = [
  {
    text: 'Guide',
    link: '/guide/getting-started',
    activeMatch: '/guide/',
  },
  {
    text: 'Reference',
    items: referenceItems(),
  },
  {
    text: 'Rust',
    items: docsRsItems(),
  },
  {
    text: 'Changelog',
    activeMatch: '/changelog/',
    items: changelogItems(),
  },
];

const socialLinks: ThemeConfig['socialLinks'] = [
  { icon: 'github', link: 'https://github.com/ferreira-tb/tauri-store' },
  { icon: 'discord', link: 'https://discord.gg/ARd7McmVNv' },
];

const defaultSidebar: ThemeConfig['sidebar'] = [
  { text: 'Getting started', link: '/guide/getting-started' },
  { text: 'Persisting state', link: '/guide/persisting-state' },
  { text: 'Synchronization', link: '/guide/synchronization' },
  { text: 'Accessing from Rust', link: '/guide/accessing-from-rust' },
];

const changelogSidebar: ThemeConfig['sidebar'] = [
  { text: 'tauri-store', link: '/changelog/tauri-store' },
  { text: 'tauri-plugin-pinia', link: '/changelog/tauri-plugin-pinia' },
];

export default defineConfig({
  title: 'tauri-store',
  description: 'Persistent stores for Tauri',
  base: '/tauri-store/',
  lang: 'en-US',
  srcDir: 'src',
  srcExclude: ['**/examples/**'],
  cleanUrls: true,
  metaChunk: true,
  sitemap: {
    hostname: 'https://tb.dev.br/tauri-store/',
  },

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
    nav,
    socialLinks,
    sidebar: {
      '/': defaultSidebar,
      '/guide/': defaultSidebar,
      '/changelog/': changelogSidebar,
    },
    search: {
      provider: 'local',
    },
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

function changelogItems() {
  const tauriStore = {
    text: 'tauri-store',
    link: '/changelog/tauri-store',
  };

  const other = plugins.map((plugin) => ({
    text: plugin.name,
    link: `/changelog/${plugin.name}`,
  }));

  return [tauriStore, ...other];
}
