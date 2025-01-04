import * as Icon from './components/icons';
import type { Mode } from './components/sidebar/mode';
import { BookOpen, Code, FileClock } from 'lucide-svelte';

export function resolvePluginIcon(plugin: TauriPlugin) {
  switch (plugin) {
    case 'tauri-plugin-pinia': {
      return Icon.Pinia;
    }
    case 'tauri-plugin-svelte': {
      return Icon.Svelte;
    }
  }
}

export function resolveHeaderIcon(mode: Mode | null) {
  switch (mode) {
    case 'changelog': {
      return FileClock;
    }
    case 'reference': {
      return Code;
    }
    case 'learn':
    default: {
      return BookOpen;
    }
  }
}
