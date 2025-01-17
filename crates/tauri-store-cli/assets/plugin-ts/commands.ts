// @ts-nocheck
import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 *
 * @example
 * ```ts
 * import { clearAutosave } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await clearAutosave();
 * ```
 */
export const clearAutosave = commands.clearAutosave('PLUGIN_NAME');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 *
 * @example
 * ```ts
 * import { getDefaultSaveStrategy } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const { strategy, interval } = await getDefaultSaveStrategy();
 * ```
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('PLUGIN_NAME');

/**
 * Directory where the stores are saved.
 *
 * @example
 * ```ts
 * import { getPASCAL_PLUGIN_NAMEPath } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const path = await getPASCAL_PLUGIN_NAMEPath();
 * ```
 */
export const getPASCAL_PLUGIN_NAMEPath = commands.getStoreCollectionPath('PLUGIN_NAME');

/**
 * Gets the save strategy used by a store.
 *
 * @example
 * ```ts
 * import { getSaveStrategy } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const { strategy, interval } = await getSaveStrategy('my-store');
 * ```
 **/
export const getSaveStrategy = commands.getSaveStrategy('PLUGIN_NAME');

/**
 * Lists all the store ids.
 *
 * @example
 * ```ts
 * import { getStoreIds } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const ids = await getStoreIds();
 * ```
 */
export const getStoreIds = commands.getStoreIds('PLUGIN_NAME');

/**
 * Path where the store is saved.
 *
 * @example
 * ```ts
 * import { getStorePath } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const path = await getStorePath('my-store');
 * ```
 **/
export const getStorePath = commands.getStorePath('PLUGIN_NAME');

/**
 * Gets the state of a store.
 *
 * @example
 * ```ts
 * import { getStoreState } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * const state = await getStoreState('counter-store');
 * ```
 */
export const getStoreState = commands.getStoreState('PLUGIN_NAME');

/**
 * Saves a store to the disk.
 *
 * @example
 * ```ts
 * import { save } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * // Save a single store.
 * await save('my-store');
 *
 * // Save some stores.
 * await save('my-store', 'my-store-2');
 * ```
 **/
export const save = commands.save('PLUGIN_NAME');

/**
 * Saves all the stores to the disk.
 *
 * @example
 * ```ts
 * import { saveAll } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await saveAll();
 * ```
 */
export const saveAll = commands.saveAll('PLUGIN_NAME');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveAllNow } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await saveAllNow();
 * ```
 */
export const saveAllNow = commands.saveAllNow('PLUGIN_NAME');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveNow } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * // Save a single store.
 * await saveNow('my-store');
 *
 * // Save some stores.
 * await saveNow('my-store', 'my-store-2');
 * ```
 */
export const saveNow = commands.saveNow('PLUGIN_NAME');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 *
 * @example
 * ```ts
 * import { setAutosave } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * // Save all the stores every 5 minutes.
 * await setAutosave(5 * 60 * 1000);
 * ```
 */
export const setAutosave = commands.setAutosave('PLUGIN_NAME');

/**
 * Sets the directory where the stores are saved.
 * This will move all *currently active* stores to the new directory.
 *
 * @example
 * ```ts
 * import { setPASCAL_PLUGIN_NAMEPath } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await setPASCAL_PLUGIN_NAMEPath('/path/to/new/directory');
 * ```
 */
export const setPASCAL_PLUGIN_NAMEPath = commands.setStoreCollectionPath('PLUGIN_NAME');

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 *
 * @example
 * ```ts
 * import { setSaveStrategy } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await setSaveStrategy('my-store', 'debounce', 1000);
 * ```
 */
export const setSaveStrategy = commands.setSaveStrategy('PLUGIN_NAME');

/**
 * Sets the store options.
 *
 * @example
 * ```ts
 * import { setStoreOptions } from 'KEBAB_PLUGIN_FULL_NAME';
 *
 * await setStoreOptions('my-store', {
 *  saveOnChange: true,
 *  saveStrategy: 'debounce',
 *  saveInterval: 1000
 * });
 * ```
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
