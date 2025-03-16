import { computed, readonly, ref } from 'vue';
import { watchImmediate } from '@vueuse/core';
import { inBrowser, useRoute } from 'vitepress';
import { JavaScript, Pinia, React, Svelte } from '../components/icon';

const pathRegex = /^\/?tauri-store\/plugin-(.+?)\//;

export function useCurrentPlugin() {
  const plugin = ref<PluginName>('tauri-store');
  const icon = computed(() => resolveIcon(plugin.value));

  if (inBrowser) {
    const route = useRoute();
    watchImmediate(route, () => {
      plugin.value = parsePluginName(route.path);
    });
  }

  return {
    plugin: readonly(plugin),
    icon,
  };
}

function parsePluginName(path: string) {
  let match = pathRegex.exec(path)?.at(1);
  match &&= `@tauri-store/${match}`;
  return (match ?? 'tauri-store') as PluginName;
}

function resolveIcon(plugin: PluginName) {
  switch (plugin) {
    case '@tauri-store/pinia':
      return Pinia;
    case '@tauri-store/svelte':
      return Svelte;
    case '@tauri-store/valtio':
      return React;
    case 'tauri-store':
    default:
      return JavaScript;
  }
}
