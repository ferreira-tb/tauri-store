import * as commands from './commands';
import { EventEmitter } from './emitter';
import type { TauriStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_HOOKS,
  DEFAULT_ON_ERROR,
  DEFAULT_SAVE_ON_CHANGE,
  DEFAULT_SAVE_ON_EXIT,
  type Fn,
  type State,
  type StoreHooks,
  type TauriStoreContract,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

export class Store<S extends State> extends BaseStore<S> implements TauriStoreContract {
  public readonly id: string;
  private readonly emitter: EventEmitter<S>;
  protected options: TauriStoreOptions<S>;

  constructor(id: string, state: S, options: TauriStoreOptions<S> = {}) {
    super();

    this.id = id;

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      clone: options.clone ?? true,
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      hooks: options.hooks ?? (DEFAULT_HOOKS as StoreHooks<S>),
      onError: options.onError ?? options.hooks?.error ?? DEFAULT_ON_ERROR,
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriStoreOptions<S>>;

    this.emitter = new EventEmitter(state, {
      clone: this.clone.bind(this),
    });
  }

  protected async load(): Promise<void> {
    const state = await commands.load<S>(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  }

  protected async unload(): Promise<void> {
    await commands.unload(this.id);
  }

  protected watch(): Fn {
    const patchBackend = this.patchBackend.bind(this);
    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return this.emitter.subscribe(fn);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return this.emitter.subscribe(fn);
    }

    return this.emitter.subscribe(patchBackend);
  }

  protected patchSelf(state: S): void {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      this.emitter.patch(_state, false);
    }
  }

  protected patchBackend(state: S): void {
    this.patchBackendHelper(commands.patch, state);
  }

  protected async setOptions(): Promise<void> {
    return this.setOptionsHelper(commands.setStoreOptions);
  }

  /**
   * Returns the current state of the store.
   */
  public state(): S {
    return this.emitter.state();
  }

  /**
   * Returns the value of the given `key`.
   */
  public get<K extends keyof S>(key: K): S[K] {
    return this.emitter.get(key);
  }

  /**
   * Inserts a key-value pair in the store.
   */
  public set<K extends keyof S>(key: K, value: S[K]): void {
    this.emitter.set(key, value, true);
  }

  /**
   * Updates a value in the store using a function.
   */
  public update<K extends keyof S>(key: K, fn: (value: S[K]) => S[K]): void {
    this.emitter.update(key, fn, true);
  }

  /**
   * Updates the store with the given a partial state.
   */
  public patch(state: Partial<S>): void {
    this.emitter.patch(state, true);
  }

  /**
   * Returns the amount of items in the store.
   */
  public size(): number {
    return this.emitter.size();
  }

  /**
   * Returns `true` if the given `key` exists in the store.
   */
  public has(key: string): boolean {
    return this.emitter.has(key);
  }

  /**
   * Returns a list of all keys in the store.
   */
  public keys(): (keyof S)[] {
    return this.emitter.keys();
  }

  /**
   * Returns a list of all values in the store.
   */
  public values(): S[keyof S][] {
    return this.emitter.values();
  }

  /**
   * Subscribes to changes in the store.
   */
  public subscribe(fn: (state: S) => void): Fn {
    return this.emitter.subscribe(fn);
  }

  /**
   * Subscribes to changes in a specific key in the store.
   */
  public subscribeKey<K extends keyof S>(key: K, fn: (value: S[K]) => void): Fn {
    return this.emitter.subscribeKey(key, fn);
  }

  public getPath(): Promise<string> {
    return commands.getStorePath(this.id);
  }

  public save(): Promise<void> {
    return commands.save(this.id);
  }

  public saveAll(): Promise<void> {
    return commands.saveAll();
  }

  public saveAllNow(): Promise<void> {
    return commands.saveAllNow();
  }

  public saveNow(): Promise<void> {
    return commands.saveNow(this.id);
  }

  private clone<T>(value: T): T {
    if (typeof this.options.clone === 'function') {
      return this.options.clone(value);
    } else if (this.options.clone) {
      return structuredClone(value);
    }

    return value;
  }
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
