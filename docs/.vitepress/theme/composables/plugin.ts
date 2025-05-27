import { inBrowser, useRoute, useRouter } from 'vitepress';
import { computed, effectScope, readonly, ref, watchEffect } from 'vue';
import { JavaScript, Pinia, React, Svelte, Vue } from '../components/icon';

const PATH_REGEX = /^\/?tauri-store\/plugin-(.+?)\//;

export function useCurrentPlugin() {
  const plugin = ref<PluginName>('tauri-store');
  const icon = computed(() => resolveIcon(plugin.value));

  if (inBrowser) {
    const scope = effectScope(true);
    scope.run(() => {
      const route = useRoute();
      const _set = () => set(route.path);
      watchEffect(_set);

      const router = useRouter();
      router.onAfterPageLoad = _set;
      router.onAfterRouteChange = _set;

      _set();
    });
  }

  function set(path: string) {
    const name = parsePluginName(path);
    if (plugin.value !== name) {
      plugin.value = name;
    }
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
    case '@tauri-store/vue':
      return Vue;
    case 'tauri-store':
    default:
      return JavaScript;
  }
}
