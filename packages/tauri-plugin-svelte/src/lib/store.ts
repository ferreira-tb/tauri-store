import { tick } from 'svelte';
import * as commands from './commands';
import type { StoreContract, TauriPluginSvelteStoreOptions } from './types';
import {
  type Subscriber,
  type Unsubscriber,
  type Updater,
  writable,
  type Writable,
} from 'svelte/store';
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
 * A [writable store] that can sync its state with the Rust backend and persist it to disk.
 * It adheres to the [Svelte store contract], so it's interchangeable with conventional Svelte stores.
 *
 * If you prefer runes, use the [`RuneStore`] class instead.
 *
 * [writable store]: https://svelte.dev/docs/svelte/stores#svelte-store-writable
 * [Svelte store contract]: https://svelte.dev/docs/svelte/stores#Store-contract
 * [`RuneStore`]: https://tb.dev.br/tauri-store/reference/tauri-plugin-svelte/classes/RuneStore.html
 *
 * @example
 * ```ts
 * import { Store } from 'tauri-plugin-svelte';
 *
 * const store = new Store('counter', { count: 0 });
 *
 * // Start the store, allowing it to sync with the backend.
 * await store.start();
 *
 * store.subscribe((state) => {
 *  console.log(state.count);
 * });
 *
 * store.set({ count: 2 });
 *
 * store.update((state) => {
 *  state.count += 1;
 * });
 *
 * // Save the store to disk.
 * await store.save();
 * ```
 */
export class Store<S extends State> extends BaseStore<S> implements StoreContract<S> {
  public readonly id: string;
  private readonly store: Writable<S>;
  protected options: TauriPluginSvelteStoreOptions<S>;
  protected override readonly flush = tick;

  constructor(id: string, state: S, options: TauriPluginSvelteStoreOptions<S> = {}) {
    super();

    this.id = id;
    this.store = writable(state);

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
    } satisfies Required<TauriPluginSvelteStoreOptions<S>>;
  }

  public set(value: S): void {
    this.store.set(value);
  }

  public update(updater: Updater<S>): void {
    this.store.update(updater);
  }

  public subscribe(run: Subscriber<S>, invalidate?: () => void): Unsubscriber {
    return this.store.subscribe(run, invalidate);
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
    // A Svelte subscriber is called immediately upon subscription.
    // If we try to sync on the first call, we'll end up with an infinite loop.
    let isFirstCall = true;
    const patchBackend = (value: S) => {
      if (isFirstCall) {
        isFirstCall = false;
        return;
      }

      this.patchBackend(value);
    };

    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return this.subscribe(fn);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return this.subscribe(fn);
    }

    return this.subscribe(patchBackend);
  }

  protected patchSelf(state: S): void {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      this.update((value) => Object.assign(value, _state));
    }
  }

  protected patchBackend(state: S): void {
    this.patchBackendHelper(commands.patch, state);
  }

  protected setOptions(): Promise<void> {
    return this.setOptionsHelper(commands.setStoreOptions);
  }

  public async getPath(): Promise<string> {
    return commands.getStorePath(this.id);
  }

  public async save(): Promise<void> {
    return commands.save(this.id);
  }

  public async saveAll(): Promise<void> {
    return commands.saveAll();
  }

  public async saveAllNow(): Promise<void> {
    return commands.saveAllNow();
  }

  public async saveNow(): Promise<void> {
    return commands.saveNow(this.id);
  }
}

/**
 * Creates a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { store, Store } from 'tauri-plugin-svelte';
 *
 * // These are equivalent.
 * const foo = new Store('foo', { value: 0 });
 * const bar = store('bar', { value: 0 });
 * ```
 */
export function store<S extends State>(
  id: string,
  state: S,
  options?: TauriPluginSvelteStoreOptions<S>
): Store<S> {
  return new Store(id, state, options);
}
