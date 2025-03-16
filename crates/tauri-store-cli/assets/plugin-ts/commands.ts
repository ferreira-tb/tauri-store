// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 *
 * @example
 * ```ts
 * import { clearAutosave } from '__PACKAGE_NAME__';
 *
 * await clearAutosave();
 * ```
 */
export const clearAutosave = commands.clearAutosave('__PLUGIN_TITLE__');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 *
 * @example
 * ```ts
 * import { getDefaultSaveStrategy } from '__PACKAGE_NAME__';
 *
 * const { strategy, interval } = await getDefaultSaveStrategy();
 * ```
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('__PLUGIN_TITLE__');

/**
 * Directory where the stores are saved.
 *
 * @example
 * ```ts
 * import { getStoreCollectionPath } from '__PACKAGE_NAME__';
 *
 * const path = await getStoreCollectionPath();
 * ```
 */
export const getStoreCollectionPath = commands.getStoreCollectionPath('__PLUGIN_TITLE__');

/**
 * Gets the save strategy used by a store.
 *
 * @example
 * ```ts
 * import { getSaveStrategy } from '__PACKAGE_NAME__';
 *
 * const { strategy, interval } = await getSaveStrategy('my-store');
 * ```
 **/
export const getSaveStrategy = commands.getSaveStrategy('__PLUGIN_TITLE__');

/**
 * Lists all the store ids.
 *
 * @example
 * ```ts
 * import { getStoreIds } from '__PACKAGE_NAME__';
 *
 * const ids = await getStoreIds();
 * ```
 */
export const getStoreIds = commands.getStoreIds('__PLUGIN_TITLE__');

/**
 * Path where the store is saved.
 *
 * @example
 * ```ts
 * import { getStorePath } from '__PACKAGE_NAME__';
 *
 * const path = await getStorePath('my-store');
 * ```
 **/
export const getStorePath = commands.getStorePath('__PLUGIN_TITLE__');

/**
 * Gets the state of a store.
 *
 * @example
 * ```ts
 * import { getStoreState } from '__PACKAGE_NAME__';
 *
 * const state = await getStoreState('my-store');
 * ```
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
 *
 * @example
 * ```ts
 * import { saveAll } from '__PACKAGE_NAME__';
 *
 * await saveAll();
 * ```
 */
export const saveAll = commands.saveAll('__PLUGIN_TITLE__');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveAllNow } from '__PACKAGE_NAME__';
 *
 * await saveAllNow();
 * ```
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
 * Sets the directory where the stores are saved.
 * This will move all *currently active* stores to the new directory.
 *
 * @example
 * ```ts
 * import { setStoreCollectionPath } from '__PACKAGE_NAME__';
 *
 * await setStoreCollectionPath('/path/to/new/directory');
 * ```
 */
export const setStoreCollectionPath = commands.setStoreCollectionPath('__PLUGIN_TITLE__');

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
export const load = commands.load('__PLUGIN_TITLE__');

/** @internal */
export const patch = commands.patch('__PLUGIN_TITLE__');

/** @internal */
export const unload = commands.unload('__PLUGIN_TITLE__');
