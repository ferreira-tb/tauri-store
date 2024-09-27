import type { MaybePromise } from '@tb-dev/utils';

export interface TauriPluginPiniaOptions {
  /** @default 0 */
  readonly debounce?: number;

  /**
   * Whether the store should be deeply watched for changes.
   * @default true
   */
  readonly deep?: boolean;

  /**
   * Custom error handler.
   * @default console.error
   */
  readonly onError?: (error: unknown) => MaybePromise<void>;
}

export type StoreKeyFilter = string | string[] | RegExp;
export type StoreKeyFilterStrategy = 'pick' | 'omit' | ((key: string) => boolean);

export interface TauriPluginPiniaStoreOptions extends TauriPluginPiniaOptions {
  /**
   * Keys the plugin should save or ignore.
   *
   * The behavior depends on the value of {@link TauriPluginPiniaStoreOptions.filterKeysStrategy}.
   */
  readonly filterKeys?: StoreKeyFilter;

  /**
   * Strategy to use when filtering keys.
   * - `pick`: Only the specified keys will be synced and saved.
   * - `omit`: All keys will be synced and saved **except** the ones specified.
   *
   * You can also provide a custom function that will be called for each key.
   * If the function returns `true`, the key will be saved and synced.
   *
   * @default 'omit'
   */
  readonly filterKeysStrategy?: StoreKeyFilterStrategy;

  /**
   * Keys the plugin should ignore. Those won't be saved nor synced.
   * @deprecated Use {@link TauriPluginPiniaStoreOptions.filterKeys} instead.
   */
  readonly ignoreKeys?: string | string[];
}

export type State = Record<string, unknown>;

/** @internal */
export interface ChangePayload {
  id: string;
  state: State;
}
