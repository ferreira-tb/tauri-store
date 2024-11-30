import type { WatchOptions } from 'vue';
import type { MaybePromise } from '@tb-dev/utils';

export interface TauriPluginPiniaOptions {
  /** @default true */
  readonly deep?: WatchOptions['deep'];
  /** @default 'pre' */
  readonly flush?: WatchOptions['flush'];

  /**
   * Custom error handler.
   * @default console.error
   */
  readonly onError?: (error: unknown) => MaybePromise<void>;

  /**
   * Interval in milliseconds to use when syncing the store with the backend.
   * This option is only valid when {@link TauriPluginPiniaOptions.syncStrategy} is set to `debounce` or `throttle`.
   *
   * @default 0
   */
  readonly syncInterval?: number;

  /**
   * Strategy to use when syncing the store with the backend.
   *
   * If the value is a number, the plugin will use `debounce` with the specified value as the interval.
   * If `null`, the strategy will be `immediate`.
   *
   * For a detailed explanation about the differences between `debounce` and `throttle`, see:
   * https://kettanaito.com/blog/debounce-vs-throttle
   *
   * @default 'immediate'
   */
  readonly syncStrategy?: 'debounce' | 'throttle' | 'immediate' | number | null;
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
}

export type State = Record<string, unknown>;

/** @internal */
export interface ChangePayload {
  id: string;
  state: State;
}
