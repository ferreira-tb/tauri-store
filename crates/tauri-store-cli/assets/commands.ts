// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 */
export const clearAutosave = commands.clearAutosave('PLUGIN_NAME');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('PLUGIN_NAME');

/**
 * Directory where the stores are saved.
 */
export const getPASCAL_PLUGIN_NAMEPath = commands.getStoreCollectionPath('PLUGIN_NAME');

/**
 * Gets the save strategy used by a store.
 **/
export const getSaveStrategy = commands.getSaveStrategy('PLUGIN_NAME');

/**
 * Lists all the store ids.
 */
export const getStoreIds = commands.getStoreIds('PLUGIN_NAME');

/**
 * Path where the store is saved.
 **/
export const getStorePath = commands.getStorePath('PLUGIN_NAME');

/** Gets the state of a store. */
export const getStoreState = commands.getStoreState('PLUGIN_NAME');

/**
 * Saves a store to the disk.
 **/
export const save = commands.save('PLUGIN_NAME');

/**
 * Saves all the stores to the disk.
 */
export const saveAll = commands.saveAll('PLUGIN_NAME');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 */
export const saveAllNow = commands.saveAllNow('PLUGIN_NAME');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 */
export const saveNow = commands.saveNow('PLUGIN_NAME');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 */
export const setAutosave = commands.setAutosave('PLUGIN_NAME');

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 */
export const setSaveStrategy = commands.setSaveStrategy('PLUGIN_NAME');

/**
 * Sets the store options.
 */
export const setStoreOptions = commands.setStoreOptions('PLUGIN_NAME');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const load = commands.load('PLUGIN_NAME');

/** @internal */
export const patch = commands.patch('PLUGIN_NAME');

/** @internal */
export const unload = commands.unload('PLUGIN_NAME');
