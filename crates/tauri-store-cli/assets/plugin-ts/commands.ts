// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 *
 * @example
 * ```ts
 * import { clearAutosave } from '__PLUGIN_FULL_NAME__';
 *
 * await clearAutosave();
 * ```
 */
export const clearAutosave = commands.clearAutosave('__REGISTERED_PLUGIN_NAME__');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 *
 * @example
 * ```ts
 * import { getDefaultSaveStrategy } from '__PLUGIN_FULL_NAME__';
 *
 * const { strategy, interval } = await getDefaultSaveStrategy();
 * ```
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('__REGISTERED_PLUGIN_NAME__');

/**
 * Directory where the stores are saved.
 *
 * @example
 * ```ts
 * import { get__STORE_COLLECTION__Path } from '__PLUGIN_FULL_NAME__';
 *
 * const path = await get__STORE_COLLECTION__Path();
 * ```
 */
export const get__STORE_COLLECTION__Path = commands.getStoreCollectionPath(
  '__REGISTERED_PLUGIN_NAME__'
);

/**
 * Gets the save strategy used by a store.
 *
 * @example
 * ```ts
 * import { getSaveStrategy } from '__PLUGIN_FULL_NAME__';
 *
 * const { strategy, interval } = await getSaveStrategy('my-store');
 * ```
 **/
export const getSaveStrategy = commands.getSaveStrategy('__REGISTERED_PLUGIN_NAME__');

/**
 * Lists all the store ids.
 *
 * @example
 * ```ts
 * import { getStoreIds } from '__PLUGIN_FULL_NAME__';
 *
 * const ids = await getStoreIds();
 * ```
 */
export const getStoreIds = commands.getStoreIds('__REGISTERED_PLUGIN_NAME__');

/**
 * Path where the store is saved.
 *
 * @example
 * ```ts
 * import { getStorePath } from '__PLUGIN_FULL_NAME__';
 *
 * const path = await getStorePath('my-store');
 * ```
 **/
export const getStorePath = commands.getStorePath('__REGISTERED_PLUGIN_NAME__');

/**
 * Gets the state of a store.
 *
 * @example
 * ```ts
 * import { getStoreState } from '__PLUGIN_FULL_NAME__';
 *
 * const state = await getStoreState('counter-store');
 * ```
 */
export const getStoreState = commands.getStoreState('__REGISTERED_PLUGIN_NAME__');

/**
 * Saves a store to the disk.
 *
 * @example
 * ```ts
 * import { save } from '__PLUGIN_FULL_NAME__';
 *
 * // Save a single store.
 * await save('my-store');
 *
 * // Save some stores.
 * await save('my-store', 'my-store-2');
 * ```
 **/
export const save = commands.save('__REGISTERED_PLUGIN_NAME__');

/**
 * Saves all the stores to the disk.
 *
 * @example
 * ```ts
 * import { saveAll } from '__PLUGIN_FULL_NAME__';
 *
 * await saveAll();
 * ```
 */
export const saveAll = commands.saveAll('__REGISTERED_PLUGIN_NAME__');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveAllNow } from '__PLUGIN_FULL_NAME__';
 *
 * await saveAllNow();
 * ```
 */
export const saveAllNow = commands.saveAllNow('__REGISTERED_PLUGIN_NAME__');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveNow } from '__PLUGIN_FULL_NAME__';
 *
 * // Save a single store.
 * await saveNow('my-store');
 *
 * // Save some stores.
 * await saveNow('my-store', 'my-store-2');
 * ```
 */
export const saveNow = commands.saveNow('__REGISTERED_PLUGIN_NAME__');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 *
 * @example
 * ```ts
 * import { setAutosave } from '__PLUGIN_FULL_NAME__';
 *
 * // Save all the stores every 5 minutes.
 * await setAutosave(5 * 60 * 1000);
 * ```
 */
export const setAutosave = commands.setAutosave('__REGISTERED_PLUGIN_NAME__');

/**
 * Sets the directory where the stores are saved.
 * This will move all *currently active* stores to the new directory.
 *
 * @example
 * ```ts
 * import { set__STORE_COLLECTION__Path } from '__PLUGIN_FULL_NAME__';
 *
 * await set__STORE_COLLECTION__Path('/path/to/new/directory');
 * ```
 */
export const set__STORE_COLLECTION__Path = commands.setStoreCollectionPath(
  '__REGISTERED_PLUGIN_NAME__'
);

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 *
 * @example
 * ```ts
 * import { setSaveStrategy } from '__PLUGIN_FULL_NAME__';
 *
 * await setSaveStrategy('my-store', 'debounce', 1000);
 * ```
 */
export const setSaveStrategy = commands.setSaveStrategy('__REGISTERED_PLUGIN_NAME__');

/**
 * Sets the store options.
 *
 * @example
 * ```ts
 * import { setStoreOptions } from '__PLUGIN_FULL_NAME__';
 *
 * await setStoreOptions('my-store', {
 *  saveOnChange: true,
 *  saveStrategy: 'debounce',
 *  saveInterval: 1000
 * });
 * ```
 */
export const setStoreOptions = commands.setStoreOptions('__REGISTERED_PLUGIN_NAME__');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const load = commands.load('__REGISTERED_PLUGIN_NAME__');

/** @internal */
export const patch = commands.patch('__REGISTERED_PLUGIN_NAME__');

/** @internal */
export const unload = commands.unload('__REGISTERED_PLUGIN_NAME__');
