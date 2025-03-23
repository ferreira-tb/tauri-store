import { inBrowser, useRoute } from 'vitepress';
import { computed, readonly, ref, watch } from 'vue';
import { JavaScript, Pinia, React, Svelte } from '../components/icon';

const PATH_REGEX = /^\/?tauri-store\/plugin-(.+?)\//;

export function useCurrentPlugin() {
  const plugin = ref<PluginName>('tauri-store');
  const icon = computed(() => resolveIcon(plugin.value));

  if (inBrowser) {
    const route = useRoute();
    watch(route, () => set(route.path), {
      deep: true,
      immediate: true,
    });

    set(route.path);
  }

  function set(path: string) {
    plugin.value = parsePluginName(path);
  }

  return {
    plugin: readonly(plugin),
    icon,
  };
}

function parsePluginName(path: string) {
  let match = PATH_REGEX.exec(path)?.at(1);
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
    case '@tauri-store/zustand':
      return React;
    case 'tauri-store':
    default:
      return JavaScript;
  }
}
