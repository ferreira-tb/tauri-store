// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 *
 * @example
 * ```ts
 * import { clearAutosave } from 'tauri-plugin-valtio';
 *
 * await clearAutosave();
 * ```
 */
export const clearAutosave = commands.clearAutosave('valtio');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 *
 * @example
 * ```ts
 * import { getDefaultSaveStrategy } from 'tauri-plugin-valtio';
 *
 * const { strategy, interval } = await getDefaultSaveStrategy();
 * console.log(strategy, interval);
 * ```
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('valtio');

/**
 * Directory where the stores are saved.
 *
 * @example
 * ```ts
 * import { getValtioPath } from 'tauri-plugin-valtio';
 *
 * const path = await getValtioPath();
 * console.log(path);
 * ```
 */
export const getValtioPath = commands.getStoreCollectionPath('valtio');

/**
 * Gets the save strategy used by a store.
 *
 * @example
 * ```ts
 * import { getSaveStrategy } from 'tauri-plugin-valtio';
 *
 * const { strategy, interval } = await getSaveStrategy('my-store');
 * console.log(strategy, interval);
 * ```
 **/
export const getSaveStrategy = commands.getSaveStrategy('valtio');

/**
 * Lists all the store ids.
 *
 * @example
 * ```ts
 * import { getStoreIds } from 'tauri-plugin-valtio';
 *
 * const ids = await getStoreIds();
 * for (const id of ids) {
 *   console.log(id);
 * }
 */
export const getStoreIds = commands.getStoreIds('valtio');

/**
 * Path where the store is saved.
 *
 * @example
 * ```ts
 * import { getStorePath } from 'tauri-plugin-valtio';
 *
 * const path = await getStorePath('my-store');
 * console.log(path);
 * ```
 **/
export const getStorePath = commands.getStorePath('valtio');

/**
 * Gets the state of a store.
 *
 * @example
 * ```ts
 * import { getStoreState } from 'tauri-plugin-valtio';
 *
 * const state = await getStoreState('counter-store');
 * console.log(state.counter);
 * ```
 */
export const getStoreState = commands.getStoreState('valtio');

/**
 * Saves a store to the disk.
 *
 * @example
 * ```ts
 * import { save } from 'tauri-plugin-valtio';
 *
 * // Save a single store.
 * await save('my-store');
 *
 * // Save some stores.
 * await save('my-store', 'my-store-2');
 * ```
 **/
export const save = commands.save('valtio');

/**
 * Saves all the stores to the disk.
 *
 * @example
 * ```ts
 * import { saveAll } from 'tauri-plugin-valtio';
 *
 * await saveAll();
 * ```
 */
export const saveAll = commands.saveAll('valtio');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveAllNow } from 'tauri-plugin-valtio';
 *
 * await saveAllNow();
 * ```
 */
export const saveAllNow = commands.saveAllNow('valtio');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveNow } from 'tauri-plugin-valtio';
 *
 * // Save a single store.
 * await saveNow('my-store');
 *
 * // Save some stores.
 * await saveNow('my-store', 'my-store-2');
 * ```
 */
export const saveNow = commands.saveNow('valtio');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 *
 * @example
 * ```ts
 * import { setAutosave } from 'tauri-plugin-valtio';
 *
 * // Save all the stores every 5 minutes.
 * await setAutosave(5 * 60 * 1000);
 * ```
 */
export const setAutosave = commands.setAutosave('valtio');

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 *
 * @example
 * ```ts
 * import { setSaveStrategy } from 'tauri-plugin-valtio';
 *
 * await setSaveStrategy('my-store', 'debounce', 1000);
 * ```
 */
export const setSaveStrategy = commands.setSaveStrategy('valtio');

/**
 * Sets the store options.
 *
 * @example
 * ```ts
 * import { setStoreOptions } from 'tauri-plugin-valtio';
 *
 * await setStoreOptions('my-store', {
 *  saveOnChange: true,
 *  saveStrategy: 'debounce',
 *  saveInterval: 1000
 * });
 * ```
 */
export const setStoreOptions = commands.setStoreOptions('valtio');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const load = commands.load('valtio');

/** @internal */
export const patch = commands.patch('valtio');

/** @internal */
export const unload = commands.unload('valtio');