import * as commands from './commands';
import type { StoreBackendOptions, StoreRef, TauriPluginVueStoreOptions } from './types';
import {
  type MaybeRefOrGetter,
  nextTick,
  type Ref,
  toRaw,
  toRef,
  watch,
  type WatchOptions,
} from 'vue';
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

const GLOBAL_STORE_ID = 'global';

export class Store<S extends State> extends BaseStore<S> {
  public readonly id: string;
  public readonly state: Ref<S>;
  protected options: TauriPluginVueStoreOptions<S>;
  protected override readonly flush = (): Promise<void> => nextTick();

  constructor(id: string, state: Ref<S>, options: TauriPluginVueStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.state = state;

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      autoStart: options.autoStart ?? DEFAULT_AUTO_START,
      deep: options.deep ?? true,
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      flush: options.flush ?? 'pre',
      hooks: merge(options.hooks, DEFAULT_HOOKS as StoreHooks<S>),
      save: options.save ?? DEFAULT_SAVE,
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      sync: options.sync ?? DEFAULT_SYNC,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginVueStoreOptions<S>>;

    void this.updateDenylist(commands);
    void this.tryAutoStart();
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
      this.patchBackend({ ...toRaw(state) });
    };

    const options: WatchOptions = {
      deep: this.options.deep,
      flush: this.options.flush,
    };

    if (this.syncStrategy === 'debounce') {
      patchBackend = debounce(patchBackend, this.syncInterval);
    } else if (this.syncStrategy === 'throttle') {
      patchBackend = throttle(patchBackend, this.syncInterval);
    }

    return watch(this.state, patchBackend, options);
  };

  protected readonly patchSelf = (state: S): void => {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      Object.assign(this.state.value, _state);
    }
  };

  protected readonly patchBackend = (state: S): void => {
    this.patchBackendHelper(commands.patch, state);
  };

  protected readonly setOptions = (): Promise<void> => {
    return this.setOptionsHelper(commands.setStoreOptions);
  };
}

/**
 * Create a new reactive store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { createStore } from '@tauri-store/vue';
 *
 * const foo = createStore('foo', { counter: 0 });
 *
 * export const increment = () => {
 *  foo.value.counter++;
 * };
 * ```
 */
export function createStore<S extends State>(
  id: string,
  state: MaybeRefOrGetter<S>,
  options: TauriPluginVueStoreOptions<S> = {}
): StoreRef<S> {
  if (id === GLOBAL_STORE_ID) {
    throw new Error(`"${GLOBAL_STORE_ID}" is a reserved id and cannot be used`);
  }

  return _createStore(id, state, options);
}

/** @internal */
function _createStore<S extends State>(
  id: string,
  state: MaybeRefOrGetter<S>,
  options: TauriPluginVueStoreOptions<S> = {}
): StoreRef<S> {
  const stateRef = toRef(state) as Ref<S>;
  const _store = new Store(id, stateRef, options);
  const $tauri: TauriStoreContract = {
    id: _store.id,
    getPath: () => commands.getStorePath(_store.id),
    save: () => commands.save(_store.id),
    saveAll: () => commands.saveAll(),
    saveAllNow: () => commands.saveAllNow(),
    saveNow: () => commands.save(_store.id),
    start: () => _store.start(),
    stop: () => _store.stop(),
    destroy: async () => {
      await commands.destroy(_store.id);
      await _store.stop();
    },
  };

  return Object.assign(stateRef, { $tauri });
}

/**
 * This is an alias for [`createStore`].
 *
 * [`createStore`]: https://tb.dev.br/tauri-store/js-docs/plugin-vue/functions/createStore.html
 */
export const store = createStore;

export const globalStore = _createStore(GLOBAL_STORE_ID, {} as State, {
  autoStart: true,
  saveOnChange: true,
  saveOnExit: true,
});

export function setGlobalStoreOptions(options: StoreBackendOptions): Promise<void> {
  return commands.setStoreOptions(GLOBAL_STORE_ID, options);
}
