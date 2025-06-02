import type { MaybePromise, Option } from './utils';
import type { LooseTimeStrategyKind, TimeStrategyRawTuple } from '../time-strategy';

/** Options that can also be updated from Rust. */
export interface StoreBackendOptions {
  /**
   * Saves the store whenever there is a state change.
   * @default false
   */
  saveOnChange?: boolean;

  /**
   * Saves the store automatically on a graceful exit.
   * @default true
   */
  saveOnExit?: boolean;

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
}

/** @internal */
export interface StoreBackendRawOptions {
  readonly saveOnChange?: Option<boolean>;
  readonly saveOnExit?: Option<boolean>;
  readonly saveStrategy?: Option<TimeStrategyRawTuple>;
}

/** Options that can only be set from JavaScript. */
export interface StoreFrontendOptions<S extends State = State> {
  /**
   * Whether to automatically start the store.
   *
   * @default false
   */
  readonly autoStart?: boolean | ((storeId: string) => MaybePromise<boolean>);

  /**
   * Keys the plugin should sync or ignore.
   *
   * The behavior depends on the value of {@link StoreFrontendOptions.filterKeysStrategy}.
   *
   * This option is ignored if you set a callback as the filter strategy.
   *
   * @default null
   */
  readonly filterKeys?: StoreKeyFilter;

  /**
   * Strategy to use when filtering keys.
   * - `pick`: Only the specified keys will be synced.
   * - `omit`: All keys will be synced **except** the ones specified.
   *
   * You can also provide a custom function that will be called for each key.
   * If the function returns `true`, the key will be synced.
   *
   * The filtering is **shallow**, meaning that nested keys will not be filtered.
   *
   * @default 'omit'
   */
  readonly filterKeysStrategy?: StoreKeyFilterStrategy;

  /**
   * Hooks to run custom logic at specific points in the store lifecycle.
   */
  readonly hooks?: StoreHooks<S>;

  /**
   * Whether this store can be saved. Setting this to `false` will add the store to the save denylist.
   *
   * @default true
   */
  readonly save?: boolean;

  /**
   * Whether this store can be synced. Setting this to `false` will add the store to the sync denylist.
   *
   * @default true
   */
  readonly sync?: boolean;

  /**
   * Interval in milliseconds to use when syncing the store with the backend.
   * This option is only valid when {@link StoreFrontendOptions.syncStrategy} is set to `debounce` or `throttle`.
   *
   * @default 0
   */
  readonly syncInterval?: number;

  /**
   * Strategy to use when syncing the store with the backend.
   *
   * Whenever there's a state change, the store sends a notification to Rust so they can stay in sync.
   * Since data gets serialized every time this happens, it can be expensive to do it too often.
   * To mitigate this, we can use `debounce` or `throttle` to control how often this synchronization occurs.
   *
   * If the value is a number, the plugin will use `debounce` with the specified value as the interval.
   * If `null` or `undefined`, the strategy will be `immediate`.
   *
   * For a detailed explanation about the differences between `debounce` and `throttle`, see:
   * https://kettanaito.com/blog/debounce-vs-throttle
   *
   * @default 'immediate'
   */
  readonly syncStrategy?: LooseTimeStrategyKind;
}

/**
 * Hooks to run custom logic at specific points in the store lifecycle.
 */
export interface StoreHooks<S extends State = State> {
  /**
   * Hook that runs **before** the store sends its state to Rust.
   * Can be used to modify the state before the sync.
   *
   * Returning a nullish value will abort the operation.
   */
  readonly beforeBackendSync?: (state: S) => Option<Partial<S>>;

  /**
   * Hook that runs **before** the store attempts to update itself with data coming from Rust.
   * Can be used to modify the state before the changes are applied.
   *
   * Returning a nullish value will abort the operation.
   */
  readonly beforeFrontendSync?: (state: S) => Option<Partial<S>>;

  /**
   * Custom error handler.
   * @default console.error
   */
  readonly error?: (error: unknown) => MaybePromise<void>;
}

/** Options to configure how the store should behave. */
export type StoreOptions<S extends State = State> = StoreBackendOptions & StoreFrontendOptions<S>;

/** A contract that a store must adhere to in order to be considered a valid implementation. */
export interface TauriStoreContract {
  /** Store id. */
  readonly id: string;

  /** Path where the store is saved. */
  readonly getPath: () => Promise<string>;
  /** Saves the store to the disk. */
  readonly save: () => Promise<void>;
  /** Saves all stores to the disk. */
  readonly saveAll: () => Promise<void>;
  /** Saves all the stores to the disk immediately, ignoring the save strategy. */
  readonly saveAllNow: () => Promise<void>;
  /** Saves the store to the disk immediately, ignoring the save strategy. */
  readonly saveNow: () => Promise<void>;
  /** Starts watching for changes. */
  readonly start: () => Promise<void>;
  /** Stops watching for changes. */
  readonly stop: () => Promise<void>;
}

/** State of a store. */
export type State = Record<string, unknown>;

/** Keys to filter. */
export type StoreKeyFilter = string | string[] | RegExp | null;

/** Strategy to use when filtering keys. */
export type StoreKeyFilterStrategy = 'pick' | 'omit' | ((key: string) => boolean);
