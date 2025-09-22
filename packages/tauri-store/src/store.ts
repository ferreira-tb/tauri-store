import * as commands from './commands';
import type { TauriStoreOptions } from './types';
import { effect, effectScope, signal } from 'alien-signals';
import {
  BaseStore,
  debounce,
  DEFAULT_AUTO_START,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_HOOKS,
  DEFAULT_SAVE,
  DEFAULT_SAVE_ON_CHANGE,
  DEFAULT_SAVE_ON_EXIT,
  DEFAULT_SYNC,
  type Fn,
  merge,
  type State,
  type StoreHooks,
  type TauriStoreContract,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

/**
 * A key-value store that can sync its state with the Rust backend and persist it to disk.
 */
export class Store<S extends State> extends BaseStore<S> implements TauriStoreContract {
  public readonly id: string;
  private readonly signal: ReturnType<typeof signal<S>>;
  protected options: TauriStoreOptions<S>;

  constructor(id: string, state: S, options: TauriStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.signal = signal(state);

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      autoStart: options.autoStart ?? DEFAULT_AUTO_START,
      clone: options.clone ?? true,
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      hooks: merge(options.hooks, DEFAULT_HOOKS as StoreHooks<S>),
      save: options.save ?? DEFAULT_SAVE,
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      sync: options.sync ?? DEFAULT_SYNC,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriStoreOptions<S>>;

    void this.updateDenylist(commands);
    void this.checkAutoStart();
  }

  protected readonly load = async (): Promise<void> => {
    const state = await commands.load<S>(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  };

  protected readonly unload = async (): Promise<void> => {
    await commands.unload(this.id);
  };

  protected readonly watch = (): Fn => {
    let patchBackend = (state: S) => {
      this.patchBackend(state);
    };

    if (this.syncStrategy === 'debounce') {
      patchBackend = debounce(patchBackend, this.syncInterval);
    } else if (this.syncStrategy === 'throttle') {
      patchBackend = throttle(patchBackend, this.syncInterval);
    }

    let isFirstCall = true;
    const subscribeFn = () => {
      const state = this.signal();
      if (isFirstCall) {
        isFirstCall = false;
        return;
      }

      patchBackend(state);
    };

    return this.subscribe(subscribeFn);
  };

  protected readonly patchSelf = (state: S): void => {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      this.patch(_state);
    }
  };

  protected readonly patchBackend = (state: S): void => {
    this.patchBackendHelper(commands.patch, state);
  };

  protected readonly setOptions = (): Promise<void> => {
    return this.setOptionsHelper(commands.setStoreOptions);
  };

  /**
   * Returns the current state of the store.
   */
  public readonly state = (): S => {
    return this.clone(this.signal());
  };

  /**
   * Returns the value of the given `key`.
   */
  public readonly get = <K extends keyof S>(key: K): S[K] => {
    return this.clone(this.signal()[key]);
  };

  /**
   * Inserts a key-value pair in the store.
   */
  public readonly set = <K extends keyof S>(key: K, value: S[K]): void => {
    this.signal({ ...this.signal(), [key]: this.clone(value) });
  };

  /**
   * Updates a value in the store using a function.
   */
  public readonly update = <K extends keyof S>(key: K, fn: (value: S[K]) => S[K]): void => {
    this.set(key, fn(this.get(key)));
  };

  /**
   * Updates the store with the given a partial state.
   */
  public readonly patch = (state: Partial<S>): void => {
    this.signal({ ...this.signal(), ...state });
  };

  /**
   * Returns the amount of items in the store.
   */
  public readonly size = (): number => {
    return Object.keys(this.signal()).length;
  };

  /**
   * Returns `true` if the given `key` exists in the store.
   */
  public readonly has = (key: string): boolean => {
    return Object.hasOwn(this.signal(), key);
  };

  /**
   * Returns a list of all keys in the store.
   */
  public readonly keys = (): string[] => {
    return Object.keys(this.signal());
  };

  /**
   * Returns a list of all values in the store.
   */
  public readonly values = (): S[keyof S][] => {
    const values = Object.values(this.signal());
    const clone = (value: unknown) => this.clone(value);
    return Array.from(values).map(clone) as S[keyof S][];
  };

  /**
   * Subscribes to changes in the store.
   */
  public readonly subscribe = (fn: (state: S) => void): Fn => {
    return effectScope(() => effect(() => fn(this.state())));
  };

  public readonly getPath = (): Promise<string> => {
    return commands.getStorePath(this.id);
  };

  public readonly save = (): Promise<void> => {
    return commands.save(this.id);
  };

  public readonly saveAll = (): Promise<void> => {
    return commands.saveAll();
  };

  public readonly saveAllNow = (): Promise<void> => {
    return commands.saveAllNow();
  };

  public readonly saveNow = (): Promise<void> => {
    return commands.saveNow(this.id);
  };

  public readonly destroy = async (): Promise<void> => {
    await commands.destroy(this.id);
    await this.stop();
  };

  private readonly clone = <T>(value: T): T => {
    if (typeof this.options.clone === 'function') {
      return this.options.clone(value);
    } else if (this.options.clone) {
      return structuredClone(value);
    }

    return value;
  };
}

/**
 * Creates a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { store, Store } from 'tauri-store';
 *
 * // These are equivalent.
 * const foo = new Store('foo', { value: 0 });
 * const bar = store('bar', { value: 0 });
 * ```
 */
export function store<S extends State>(
  id: string,
  state: S,
  options?: TauriStoreOptions<S>
): Store<S> {
  return new Store(id, state, options);
}
