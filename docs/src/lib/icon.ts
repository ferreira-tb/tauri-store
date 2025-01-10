import * as Icon from '$lib/components/icons';

export function resolvePluginIcon(plugin: TauriPlugin) {
  switch (plugin) {
    case 'tauri-plugin-pinia': {
      return Icon.Pinia;
    }
    case 'tauri-plugin-svelte': {
      return Icon.Svelte;
    }
    case 'tauri-plugin-valtio': {
      return Icon.React;
    }
  }
}
