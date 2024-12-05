import { invoke } from '@tauri-apps/api/core';
import type { State } from '@tauri-store/shared';

/** Abort the autosave. */
export function clearAutosave(): Promise<void> {
  return invoke('plugin:pinia|clear_autosave');
}

/** Directory where the stores are saved. */
export function getPiniaPath(): Promise<string> {
  return invoke('plugin:pinia|get_pinia_path');
}

/** List all the store ids. */
export function getStoreIds(): Promise<string[]> {
  return invoke('plugin:pinia|get_store_ids');
}

/**
 * Path where the store is saved.
 * @param id The store id.
 **/
export function getStorePath(id: string): Promise<string> {
  return invoke('plugin:pinia|get_store_path', { id });
}

/** Get the state of a store. */
export function getStoreState<T extends State>(id: string): Promise<T | null> {
  return invoke('plugin:pinia|get_store_state', { id });
}

/**
 * Save a store to the disk.
 * @param id The store id.
 **/
export function save(...id: (string | string[])[]): Promise<void> {
  const stores = id.flat(Number.POSITIVE_INFINITY);
  return invoke('plugin:pinia|save_some', { ids: stores });
}

/** Save all the stores to the disk. */
export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}

/**
 * Save the stores periodically.
 * @param interval The interval in milliseconds.
 */
export function setAutosave(interval: number | null): Promise<void> {
  if (typeof interval === 'number' && interval > 0) {
    return invoke('plugin:pinia|set_autosave', { interval });
  }

  return clearAutosave();
}
