import { invoke } from '@tauri-apps/api/core';

/**
 * Stop the autosave.
 *
 * **WARNING**: This requires the `unstable-async` feature to be enabled.
 */
export function clearAutosave(): Promise<void> {
  return invoke('plugin:pinia|clear_autosave');
}

/** Add a store to the sync denylist. */
export function disableSync(id: string): Promise<void> {
  return invoke('plugin:pinia|disable_sync', { id });
}

/** Remove a store from the sync denylist. */
export function enableSync(id: string): Promise<void> {
  return invoke('plugin:pinia|enable_sync', { id });
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

/**
 * Save a store to the disk.
 * @param id The store id.
 **/
export function save(id: string): Promise<void> {
  return invoke('plugin:pinia|save', { id });
}

/**
 * Save some stores to the disk.
 * @param ids The store ids.
 **/
export function saveSome(ids: string[]): Promise<void> {
  return invoke('plugin:pinia|save_some', { ids });
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
