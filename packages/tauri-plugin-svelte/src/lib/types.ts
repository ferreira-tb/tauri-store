import type { Writable } from 'svelte/store';
import type { State, StoreOptions, TauriStoreContract } from '@tauri-store/shared';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriStoreContract,
} from '@tauri-store/shared';

/**
 * Options for the Svelte store.
 */
export type TauriPluginSvelteStoreOptions<S extends State> = StoreOptions<S>;

/**
 * Implementing `Writable<S>` ensures that the store also adheres to the Svelte store contract.
 *
 * @see https://svelte.dev/docs/svelte/stores#Store-contract
 */
export type StoreContract<S extends State> = TauriStoreContract & Writable<S>;

/**
 * Options for the Svelte rune store.
 */
export interface TauriPluginSvelteRuneStoreOptions<S extends State> extends StoreOptions<S> {
  /**
   * @see https://svelte.dev/docs/svelte/$effect#$effect.pre
   * @default 'post'
   */
  flush?: 'post' | 'pre';
}
