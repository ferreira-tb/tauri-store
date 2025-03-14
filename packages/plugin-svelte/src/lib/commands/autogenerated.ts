// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

import { commands } from '@tauri-store/shared';

/**
 * Stops the autosave.
 *
 * @example
 * ```ts
 * import { clearAutosave } from '@tauri-store/svelte';
 *
 * await clearAutosave();
 * ```
 */
export const clearAutosave = commands.clearAutosave('svelte');

/**
 * Gets the default save strategy for the stores.
 * It can be overridden on a per-store basis.
 *
 * @example
 * ```ts
 * import { getDefaultSaveStrategy } from '@tauri-store/svelte';
 *
 * const { strategy, interval } = await getDefaultSaveStrategy();
 * ```
 */
export const getDefaultSaveStrategy = commands.getDefaultSaveStrategy('svelte');

/**
 * Directory where the stores are saved.
 *
 * @example
 * ```ts
 * import { getStoreCollectionPath } from '@tauri-store/svelte';
 *
 * const path = await getStoreCollectionPath();
 * ```
 */
export const getStoreCollectionPath = commands.getStoreCollectionPath('svelte');

/**
 * Gets the save strategy used by a store.
 *
 * @example
 * ```ts
 * import { getSaveStrategy } from '@tauri-store/svelte';
 *
 * const { strategy, interval } = await getSaveStrategy('my-store');
 * ```
 **/
export const getSaveStrategy = commands.getSaveStrategy('svelte');

/**
 * Lists all the store ids.
 *
 * @example
 * ```ts
 * import { getStoreIds } from '@tauri-store/svelte';
 *
 * const ids = await getStoreIds();
 * ```
 */
export const getStoreIds = commands.getStoreIds('svelte');

/**
 * Path where the store is saved.
 *
 * @example
 * ```ts
 * import { getStorePath } from '@tauri-store/svelte';
 *
 * const path = await getStorePath('my-store');
 * ```
 **/
export const getStorePath = commands.getStorePath('svelte');

/**
 * Gets the state of a store.
 *
 * @example
 * ```ts
 * import { getStoreState } from '@tauri-store/svelte';
 *
 * const state = await getStoreState('my-store');
 * ```
 */
export const getStoreState = commands.getStoreState('svelte');

/**
 * Saves a store to the disk.
 *
 * @example
 * ```ts
 * import { save } from '@tauri-store/svelte';
 *
 * // Save a single store.
 * await save('my-store');
 *
 * // Save some stores.
 * await save('my-store', 'my-store-2');
 * ```
 **/
export const save = commands.save('svelte');

/**
 * Saves all the stores to the disk.
 *
 * @example
 * ```ts
 * import { saveAll } from '@tauri-store/svelte';
 *
 * await saveAll();
 * ```
 */
export const saveAll = commands.saveAll('svelte');

/**
 * Saves all the stores to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveAllNow } from '@tauri-store/svelte';
 *
 * await saveAllNow();
 * ```
 */
export const saveAllNow = commands.saveAllNow('svelte');

/**
 * Saves a store to the disk immediately, ignoring the save strategy.
 *
 * @example
 * ```ts
 * import { saveNow } from '@tauri-store/svelte';
 *
 * // Save a single store.
 * await saveNow('my-store');
 *
 * // Save some stores.
 * await saveNow('my-store', 'my-store-2');
 * ```
 */
export const saveNow = commands.saveNow('svelte');

/**
 * Saves the stores periodically.
 * @param interval The interval in milliseconds.
 *
 * @example
 * ```ts
 * import { setAutosave } from '@tauri-store/svelte';
 *
 * // Save all the stores every 5 minutes.
 * await setAutosave(5 * 60 * 1000);
 * ```
 */
export const setAutosave = commands.setAutosave('svelte');

/**
 * Sets the directory where the stores are saved.
 * This will move all *currently active* stores to the new directory.
 *
 * @example
 * ```ts
 * import { setStoreCollectionPath } from '@tauri-store/svelte';
 *
 * await setStoreCollectionPath('/path/to/new/directory');
 * ```
 */
export const setStoreCollectionPath = commands.setStoreCollectionPath('svelte');

/**
 * Sets the save strategy for a store.
 * Calling this will abort any pending save operation.
 *
 * @example
 * ```ts
 * import { setSaveStrategy } from '@tauri-store/svelte';
 *
 * await setSaveStrategy('my-store', 'debounce', 1000);
 * ```
 */
export const setSaveStrategy = commands.setSaveStrategy('svelte');

/**
 * Sets the store options.
 *
 * @example
 * ```ts
 * import { setStoreOptions } from '@tauri-store/svelte';
 *
 * await setStoreOptions('my-store', {
 *  saveOnChange: true,
 *  saveStrategy: 'debounce',
 *  saveInterval: 1000
 * });
 * ```
 */
export const setStoreOptions = commands.setStoreOptions('svelte');

/////////////////////////
// INTERNAL COMMANDS
/////////////////////////

/** @internal */
export const load = commands.load('svelte');

/** @internal */
export const patch = commands.patch('svelte');

/** @internal */
export const unload = commands.unload('svelte');
