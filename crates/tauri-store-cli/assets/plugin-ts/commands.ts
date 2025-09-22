// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Removes one or more stores from the save denylist.
 */
export const allowSave = commands.allowSave('__PLUGIN_TITLE__');

/**
 * Removes one or more stores from the sync denylist.
 */
export const allowSync = commands.allowSync('__PLUGIN_TITLE__');

/**
 * Stops the autosave.
 */
export const clearAutosave = commands.clearAutosave('__PLUGIN_TITLE__');

/**
 * Adds one or more stores to the save denylist.
 */
export const denySave = commands.denySave('__PLUGIN_TITLE__');

/**
 * Adds one or more stores to the sync denylist.
 */
export const denySync = commands.denySync('__PLUGIN_TITLE__');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('__PLUGIN_TITLE__');

/**
 * Directory where the stores are saved.
 */
export const getStoreCollectionPath = commands.getStoreCollectionPath('__PLUGIN_TITLE__');

/**
 * Gets the save strategy used by a store.
 **/
export const getSaveStrategy = commands.getSaveStrategy('__PLUGIN_TITLE__');

/**
 * Lists all the store ids.
 */
export const getStoreIds = commands.getStoreIds('__PLUGIN_TITLE__');

/**
 * Path where the store is saved.
 **/
export const getStorePath = commands.getStorePath('__PLUGIN_TITLE__');

/**
 * Gets the state of a store.
 */
export const getStoreState = commands.getStoreState('__PLUGIN_TITLE__');

/**
 * Saves a store to the disk.
 *
 * @example
 * ```ts
 * import { save } from '__PACKAGE_NAME__';
 *
 * // Save a single store.
 * await save('my-store');
 *
 * // Save some stores.
 * await save('my-store', 'my-store-2');
 * ```
 **/
export const save = commands.save('__PLUGIN_TITLE__');

/**
 * Saves all the stores to the disk.
 */
export const saveAll = commands.saveAll('__PLUGIN_TITLE__');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 */
export const saveAllNow = commands.saveAllNow('__PLUGIN_TITLE__');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveNow } from '__PACKAGE_NAME__';
 *
 * // Save a single store.
 * await saveNow('my-store');
 *
 * // Save some stores.
 * await saveNow('my-store', 'my-store-2');
 * ```
 */
export const saveNow = commands.saveNow('__PLUGIN_TITLE__');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 *
 * @example
 * ```ts
 * import { setAutosave } from '__PACKAGE_NAME__';
 *
 * // Save all the stores every 5 minutes.
 * await setAutosave(5 * 60 * 1000);
 * ```
 */
export const setAutosave = commands.setAutosave('__PLUGIN_TITLE__');

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 *
 * @example
 * ```ts
 * import { setSaveStrategy } from '__PACKAGE_NAME__';
 *
 * await setSaveStrategy('my-store', 'debounce', 1000);
 * ```
 */
export const setSaveStrategy = commands.setSaveStrategy('__PLUGIN_TITLE__');

/**
 * Sets the store options.
 *
 * @example
 * ```ts
 * import { setStoreOptions } from '__PACKAGE_NAME__';
 *
 * await setStoreOptions('my-store', {
 *  saveOnChange: true,
 *  saveStrategy: 'debounce',
 *  saveInterval: 1000
 * });
 * ```
 */
export const setStoreOptions = commands.setStoreOptions('__PLUGIN_TITLE__');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const destroy = commands.destroy('__PLUGIN_TITLE__');

/** @internal */
export const load = commands.load('__PLUGIN_TITLE__');

/** @internal */
export const patch = commands.patch('__PLUGIN_TITLE__');

/** @internal */
export const unload = commands.unload('__PLUGIN_TITLE__');
