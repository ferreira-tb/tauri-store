import { invoke } from '@tauri-apps/api/core';

/**
 * Stops the autosave.
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

/** Lists all the store ids. */
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

/**
 * Saves a store to the disk.
 * @param id The store id.
 **/
export function save(id: string): Promise<void> {
  return invoke('plugin:pinia|save', { id });
}

/**
 * Saves some stores to the disk.
 * @param ids The store ids.
 **/
export function saveSome(ids: string[]): Promise<void> {
  return invoke('plugin:pinia|save_some', { ids });
}

/** Saves all the stores to the disk. */
export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}

/**
 * Saves the stores periodically.
 *
 * **WARNING**: This requires the `unstable-async` feature to be enabled.
 * @param interval The interval in milliseconds.
 */
export function setAutosave(interval: number): Promise<void> {
  return invoke('plugin:pinia|set_autosave', { interval });
}
