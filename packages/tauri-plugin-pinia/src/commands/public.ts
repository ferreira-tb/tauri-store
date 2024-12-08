import { invoke } from '@tauri-apps/api/core';
import { commands } from '@tauri-store/shared';

/** Abort the autosave. */
export const clearAutosave = commands.clearAutosave('pinia');

/** Directory where the stores are saved. */
export function getPiniaPath(): Promise<string> {
  return invoke('plugin:pinia|get_pinia_path');
}

/** List all the store ids. */
export const getStoreIds = commands.getStoreIds('pinia');

/**
 * Path where the store is saved.
 * @param id The store id.
 **/
export const getStorePath = commands.getStorePath('pinia');

/** Get the state of a store. */
export const getStoreState = commands.getStoreState('pinia');

/**
 * Save a store to the disk.
 * @param id The store id.
 **/
export const save = commands.save('pinia');

/** Save all the stores to the disk. */
export const saveAll = commands.saveAll('pinia');

/**
 * Save the stores periodically.
 * @param interval The interval in milliseconds.
 */
export const setAutosave = commands.setAutosave('pinia');
