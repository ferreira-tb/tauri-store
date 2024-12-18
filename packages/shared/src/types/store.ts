import type { Option } from '@tb-dev/utils';
import type { PluginOptions } from './plugin';
import type { LooseTimeStrategyKind, TimeStrategyRawTuple } from '../utils/time-strategy';

export interface StoreBackendOptions {
  /**
   * Interval in milliseconds to use when saving the store.
   * This option is only valid when {@link StoreBackendOptions.saveStrategy} is set to `debounce` or `throttle`.
   *
   * @default 0
   */
  saveInterval?: number;

  /**
   * Strategy to use when saving the store.
   *
   * For a detailed explanation about the differences between `debounce` and `throttle`, see:
   * https://kettanaito.com/blog/debounce-vs-throttle
   *
   * @default 'immediate'
   */
  saveStrategy?: LooseTimeStrategyKind;

  /**
   * Save the store whenever there is a state change.
   * @default false
   */
  saveOnChange?: boolean;
}

/** @internal */
export interface StoreBackendRawOptions {
  readonly saveOnChange?: Option<boolean>;
  readonly saveStrategy?: Option<TimeStrategyRawTuple>;
}

export interface StoreOptions extends StoreBackendOptions, PluginOptions {
  /**
   * Keys the plugin should save or ignore.
   *
   * The behavior depends on the value of {@link StoreOptions.filterKeysStrategy}.
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
   * The filtering is **shallow**, meaning that nested keys will not be filtered.
   *
   * @default 'omit'
   */
  readonly filterKeysStrategy?: StoreKeyFilterStrategy;
}

export interface CustomStoreProperties {
  /** Path where the store is saved. */
  readonly getPath: () => Promise<string>;
  /** Saves the store to the disk. */
  readonly save: () => Promise<void>;
  /** Saves all stores to the disk. */
  readonly saveAll: () => Promise<void>;
  /** Starts watching for changes. */
  readonly start: () => Promise<void>;
  /** Stops watching for changes. */
  readonly stop: () => Promise<void>;
}

/** State of a store. */
export type State = Record<string, unknown>;

export type StoreKeyFilter = string | string[] | RegExp;

export type StoreKeyFilterStrategy = 'pick' | 'omit' | ((key: string) => boolean);
