import type { State, StoreOptions } from '@tauri-store/shared';

export type {
  LooseTimeStrategyKind,
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriStoreContract,
  TimeStrategyKind,
} from '@tauri-store/shared';

export type CloneFn = <T>(value: T) => T;

/**
 * Options for `tauri-store`.
 */
export interface TauriStoreOptions<S extends State> extends StoreOptions<S> {
  /**
   * Whether to clone values whenever they are read or written.
   * By default, the state is cloned using [`structuredClone`].
   *
   * As this may have performance implications, you can provide a custom function
   * or disable it entirely by setting this option to `false`.
   *
   * [`structuredClone`]: https://developer.mozilla.org/en-US/docs/Web/API/Window/structuredClone
   */
  clone?: CloneFn | boolean;
}
