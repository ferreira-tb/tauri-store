import { invoke } from '@tauri-apps/api/core';
import type { State } from '@tauri-store/shared';

/**
 * Stop the autosave.
 *
 * **WARNING**: This requires the `unstable-async` feature to be enabled.
 */
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
export function save(id: string): Promise<void>;
/**
 * Save some stores to the disk.
 * @param ids The store ids.
 **/
export function save(ids: string[]): Promise<void>;
export function save(id: string | string[]): Promise<void> {
  if (typeof id === 'string') {
    return invoke('plugin:pinia|save', { id });
  }

  return invoke('plugin:pinia|save_some', { ids: id });
}

/** Save all the stores to the disk. */
export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}

/**
 * Save the stores periodically.
 *
 * **WARNING**: This requires the `unstable-async` feature to be enabled.
 * @param interval The interval in milliseconds.
 */
export function setAutosave(interval: number): Promise<void> {
  return invoke('plugin:pinia|set_autosave', { interval });
}
