import { findMetadata } from '$lib/data';
import metadata from '$lib/data/metadata.json';
import { persistent } from '$lib/stores/persistent';
import { derived, type Subscriber } from 'svelte/store';

const STORAGE_KEY = 'current-plugin';
export const DEFAULT_PLUGIN = 'tauri-plugin-pinia';

export type Metadata = (typeof metadata)[0];

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

export const currentMetadata = derived(currentPlugin, derive, findMetadata(DEFAULT_PLUGIN));

function derive(current: TauriPlugin | null) {
  return findMetadata(current ?? DEFAULT_PLUGIN);
}
