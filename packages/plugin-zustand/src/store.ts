import * as commands from './commands';
import type { StoreApi } from 'zustand';
import type { TauriPluginZustandStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  DEFAULT_AUTO_START,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_HOOKS,
  DEFAULT_SAVE_ON_CHANGE,
  DEFAULT_SAVE_ON_EXIT,
  type Fn,
  merge,
  type State,
  type StoreHooks,
  type TauriStoreContract,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

class TauriStore<S extends State, Store extends StoreApi<S>>
  extends BaseStore<S>
  implements TauriStoreContract
{
  public readonly id: string;
  public readonly store: Store;
  protected options: TauriPluginZustandStoreOptions<S>;

  constructor(id: string, store: Store, options: TauriPluginZustandStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.store = store;

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      autoStart: options.autoStart ?? DEFAULT_AUTO_START,
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      hooks: merge(options.hooks, DEFAULT_HOOKS as StoreHooks<S>),
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginZustandStoreOptions<S>>;

    void this.checkAutoStart();
  }

  protected readonly load = async (): Promise<void> => {
    const state = await commands.load<S>(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
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

    return this.store.subscribe((state) => {
      patchBackend(state);
    });
  };

  protected readonly patchSelf = (state: S): void => {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      this.store.setState(_state, /* replace */ false);
    }
  };

  protected readonly patchBackend = (state: S): void => {
    this.patchBackendHelper(commands.patch, state);
  };

  protected readonly setOptions = (): Promise<void> => {
    return this.setOptionsHelper(commands.setStoreOptions);
  };

  protected readonly unload = async (): Promise<void> => {
    await commands.unload(this.id);
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
}

/**
 * Create a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { create } from 'zustand';
 * import { createTauriStore } from '@tauri-store/zustand';
 *
 * type State = {
 *  counter: number;
 * };
 *
 * type Action = {
 *  increase: () => void;
 * };
 *
 * const useCounterStore = create<Action & State>((set) => ({
 *  counter: 0,
 *  increase: () => set((state) => ({ counter: state.counter + 1 })),
 * }))
 *
 * const tauriHandler = createTauriStore('counter-store', useCounterStore);
 * ```
 */
export function createTauriStore<S extends State, Store extends StoreApi<S>>(
  id: string,
  store: Store,
  options: TauriPluginZustandStoreOptions<S> = {}
): TauriStore<S, Store> {
  return new TauriStore(id, store, options);
}

/**
 * This is an alias for [`createTauriStore`].
 *
 * [`createTauriStore`]: https://tb.dev.br/tauri-store/js-docs/plugin-zustand/functions/createTauriStore.html
 */
export const tauri = createTauriStore;

export type { TauriStore };
