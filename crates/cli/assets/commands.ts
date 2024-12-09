// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Abort the autosave.
 */
export const clearAutosave = commands.clearAutosave('PLUGIN_NAME');

/**
 * Get the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('PLUGIN_NAME');

/**
 * Directory where the stores are saved.
 */
export const getPASCAL_PLUGIN_NAMEPath = commands.getStoreCollectionPath('PLUGIN_NAME');

/**
 * List all the store ids.
 */
export const getStoreIds = commands.getStoreIds('PLUGIN_NAME');

/**
 * Path where the store is saved.
 **/
export const getStorePath = commands.getStorePath('PLUGIN_NAME');

/**
 * Get the save strategy used by a store.
 **/
export const getStoreSaveStrategy = commands.getStoreSaveStrategy('PLUGIN_NAME');

/** Get the state of a store. */
export const getStoreState = commands.getStoreState('PLUGIN_NAME');

/**
 * Save a store to the disk.
 **/
export const save = commands.save('PLUGIN_NAME');

/**
 * Save all the stores to the disk.
 */
export const saveAll = commands.saveAll('PLUGIN_NAME');

/**
 * Save the stores periodically.
 * @param interval The interval in milliseconds.
 */
export const setAutosave = commands.setAutosave('PLUGIN_NAME');

/**
 * Sets the save strategy for a store.
 */
export const setStoreSaveStrategy = commands.setStoreSaveStrategy('PLUGIN_NAME');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const load = commands.load('PLUGIN_NAME');

/** @internal */
export const patch = commands.patch('PLUGIN_NAME');

/** @internal */
export const unload = commands.unload('PLUGIN_NAME');
