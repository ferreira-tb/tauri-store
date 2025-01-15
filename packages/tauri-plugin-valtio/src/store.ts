import * as commands from './commands';
import { proxy, snapshot, subscribe } from 'valtio';
import type { StoreBuilderReturn, TauriPluginValtioStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_ON_ERROR,
  DEFAULT_SAVE_ON_CHANGE,
  type State,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

export class Store<S extends State> extends BaseStore<S> {
  public readonly id: string;
  public readonly state: S;
  protected options: TauriPluginValtioStoreOptions;

  constructor(id: string, state: S, options: TauriPluginValtioStoreOptions = {}) {
    super();

    this.id = id;
    this.state = proxy(state);

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      onError: options.onError ?? DEFAULT_ON_ERROR,
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginValtioStoreOptions>;
  }

  protected async load() {
    const state = await commands.load<S>(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  }

  protected async unload() {
    await commands.unload(this.id);
  }

  protected watch() {
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

  protected patchSelf(state: S) {
    const _state = this.applyKeyFilters(state);
    Object.assign(this.state, _state);
  }

  protected patchBackend(state: S) {
    if (this.enabled) {
      const _state = this.applyKeyFilters(state);
      commands.patch(this.id, _state).catch((err) => this.onError?.(err));
    }
  }

  protected async setOptions() {
    try {
      await commands.setStoreOptions(this.id, {
        saveInterval: this.options.saveInterval,
        saveStrategy: this.options.saveStrategy,
        saveOnChange: this.options.saveOnChange,
      });
    } catch (err) {
      this.onError?.(err);
    }
  }
}

/**
 * Create a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { store } from 'tauri-plugin-valtio';
 *
 * export const foo = store('foo', { count: 0 });
 *
 * // "state" is the actual valtio proxy.
 * export const increment = () => {
 *  foo.state.count++;
 * };
 * ```
 */
export function store<S extends State>(
  id: string,
  state: S,
  options: TauriPluginValtioStoreOptions = {}
): StoreBuilderReturn<S> {
  const _store = new Store(id, state, options);
  return {
    state: _store.state,
    getPath: () => commands.getStorePath(_store.id),
    save: () => commands.save(_store.id),
    saveAll: () => commands.saveAll(),
    saveAllNow: () => commands.saveAllNow(),
    saveNow: () => commands.save(_store.id),
    start: () => _store.start(),
    stop: () => _store.stop(),
  };
}
