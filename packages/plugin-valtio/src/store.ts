import * as commands from './commands';
import { snapshot, subscribe, proxy as toProxy } from 'valtio';
import type { StoreBuilderReturn, TauriPluginValtioStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_HOOKS,
  DEFAULT_SAVE_ON_CHANGE,
  DEFAULT_SAVE_ON_EXIT,
  type Fn,
  merge,
  type State,
  type StoreHooks,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

/**
 * Wrapper for the Valtio proxy state.
 */
class Store<S extends State> extends BaseStore<S> {
  public readonly id: string;
  public readonly state: S;
  protected options: TauriPluginValtioStoreOptions<S>;

  constructor(id: string, proxy: S, options: TauriPluginValtioStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.state = proxy;

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      hooks: merge(options.hooks, DEFAULT_HOOKS as StoreHooks<S>),
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginValtioStoreOptions<S>>;
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
    const patchBackend = () => {
      const state = snapshot(this.state);
      this.patchBackend(state as S);
    };

    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return subscribe(this.state, fn);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return subscribe(this.state, fn);
    }

    return subscribe(this.state, patchBackend);
  }

  protected patchSelf(state: S): void {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      Object.assign(this.state, _state);
    }
  }

  protected patchBackend(state: S): void {
    this.patchBackendHelper(commands.patch, state);
  }

  protected async setOptions(): Promise<void> {
    return this.setOptionsHelper(commands.setStoreOptions);
  }
}

/**
 * Create a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { store } from '@tauri-store/valtio';
 *
 * export const foo = store('foo', { counter: 0 });
 *
 * // "state" is the actual valtio proxy.
 * export const increment = () => {
 *  foo.state.counter++;
 * };
 * ```
 */
export function store<S extends State>(
  id: string,
  state: S,
  options: TauriPluginValtioStoreOptions<S> = {}
): StoreBuilderReturn<S> {
  const proxy = toProxy(state);
  return toStore(id, proxy, options);
}

/**
 * Create a new store with the given `id` from an existing proxy state.
 *
 * @example
 * ```ts
 * import { proxy } from 'valtio';
 * import { toStore } from '@tauri-store/valtio';
 *
 * const state = proxy({ counter: 0 });
 * const foo = toStore('foo', state);
 * ```
 */
export function toStore<S extends State>(
  id: string,
  proxy: S,
  options: TauriPluginValtioStoreOptions<S> = {}
): StoreBuilderReturn<S> {
  const _store = new Store(id, proxy, options);
  return {
    state: proxy,
    getPath: () => commands.getStorePath(_store.id),
    save: () => commands.save(_store.id),
    saveAll: () => commands.saveAll(),
    saveAllNow: () => commands.saveAllNow(),
    saveNow: () => commands.save(_store.id),
    start: () => _store.start(),
    stop: () => _store.stop(),
  };
}

export type { Store };
