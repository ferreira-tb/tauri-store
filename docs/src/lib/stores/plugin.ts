import { persistent } from './persistent';
import type { Subscriber } from 'svelte/store';
import metadata from '$lib/data/metadata.json';

const DEFAULT_PLUGIN = 'tauri-plugin-pinia';
const STORAGE_KEY = 'current-plugin';

class CurrentPlugin {
  private readonly plugin = persistent<TauriPlugin>(STORAGE_KEY, DEFAULT_PLUGIN);

  public subscribe(run: Subscriber<TauriPlugin | null>, invalidate?: () => void) {
    return this.plugin.subscribe(run, invalidate);
  }

  public set(plugin: string) {
    if (metadata.some(({ name }) => name === plugin)) {
      this.plugin.set(plugin as TauriPlugin);
    }
  }
}

export const currentPlugin = new CurrentPlugin();
