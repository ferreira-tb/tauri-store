import * as Icon from '$components/icon';

export function resolveIcon(plugin: TauriPlugin) {
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
    case 'tauri-store': {
      return Icon.Javascript;
    }
  }
}

export function worksWith(plugin: TauriPlugin) {
  switch (plugin) {
    case 'tauri-plugin-pinia': {
      return ['Vue', 'Nuxt'];
    }
    case 'tauri-plugin-svelte': {
      return ['Svelte'];
    }
    case 'tauri-plugin-valtio': {
      return ['React'];
    }
    case 'tauri-store': {
      return ['Everything'];
    }
  }
}
