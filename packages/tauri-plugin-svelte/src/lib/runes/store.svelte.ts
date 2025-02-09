import { tick } from 'svelte';
import * as commands from '../commands';
import type { TauriPluginSvelteRuneStoreOptions } from '../types';
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

type Flush = TauriPluginSvelteRuneStoreOptions<State>['flush'];

const DEFAULT_FLUSH: NonNullable<Flush> = 'post';

/**
 * A reactive store that can sync its state with the Rust backend and persist it to disk.
 *
 * @example
 * ```ts
 * import { RuneStore } from 'tauri-plugin-svelte';
 *
 * const store = new RuneStore('counter', { count: 0 });
 *
 * // Start the store, allowing it to sync with the backend.
 * await store.start();
 *
 * export function increment() {
 *  store.state.count += 1;
 * }
 * ```
 */
export class RuneStore<S extends State> extends BaseStore<S> implements TauriStoreContract {
  public readonly id: string;
  public readonly state: S = $state()!;
  protected options: TauriPluginSvelteRuneStoreOptions<S>;
  protected override readonly flush = tick;

  constructor(id: string, state: S, options: TauriPluginSvelteRuneStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.state = state;

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      flush: options.flush ?? DEFAULT_FLUSH,
      hooks: options.hooks ?? (DEFAULT_HOOKS as StoreHooks<S>),
      onError: options.onError ?? options.hooks?.error ?? DEFAULT_ON_ERROR,
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginSvelteRuneStoreOptions<S>>;
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
    let patchBackend = (value: S) => {
      this.patchBackend(value);
    };

    if (this.syncStrategy === 'debounce') {
      patchBackend = debounce(patchBackend, this.syncInterval);
    } else if (this.syncStrategy === 'throttle') {
      patchBackend = throttle(patchBackend, this.syncInterval);
    }

    let isFirstCall = true;
    const effectFn = () => {
      // Effects depend only on the values they read during their last run.
      // Taking the snapshot immediately ensures that `state` is always captured.
      const snapshot = $state.snapshot(this.state) as S;
      if (isFirstCall) {
        isFirstCall = false;
        return;
      }

      patchBackend(snapshot);
    };

    return createEffectRoot(effectFn, this.options.flush);
  };

  protected readonly patchSelf = (state: S): void => {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      Object.assign(this.state, _state);
    }
  };

  protected readonly patchBackend = (state: S): void => {
    this.patchBackendHelper(commands.patch, state);
  };

  protected readonly setOptions = (): Promise<void> => {
    return this.setOptionsHelper(commands.setStoreOptions);
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

function createEffectRoot(fn: Fn, flush: Flush = DEFAULT_FLUSH): Fn {
  return $effect.root(() => {
    switch (flush) {
      case 'post':
        $effect(fn);
        break;
      case 'pre':
        $effect.pre(fn);
        break;
    }
  });
}

/**
 * Creates a new reactive store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { runeStore, RuneStore } from 'tauri-plugin-svelte';
 *
 * // These are equivalent.
 * const foo = new RuneStore('foo', { value: 0 });
 * const bar = runeStore('bar', { value: 0 });
 * ```
 */
export function runeStore<S extends State>(
  id: string,
  state: S,
  options: TauriPluginSvelteRuneStoreOptions<S> = {}
): RuneStore<S> {
  return new RuneStore(id, state, options);
}
