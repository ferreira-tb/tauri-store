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
  DEFAULT_ON_ERROR,
  DEFAULT_SAVE_ON_CHANGE,
  type State,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

/**
 * A [writable store](https://svelte.dev/docs/svelte/stores#svelte-store-writable)
 * that can sync its state with the Rust backend and persist it to disk.
 * It adheres to the [Svelte store contract](https://svelte.dev/docs/svelte/stores#Store-contract),
 * so it's interchangeable with conventional Svelte stores.
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
  private readonly store: Writable<S>;
  protected options: TauriPluginSvelteStoreOptions;
  protected override readonly flush = tick;

  constructor(
    public readonly id: string,
    state: S,
    options: TauriPluginSvelteStoreOptions = {}
  ) {
    super();

    this.store = writable(state);

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
    } satisfies Required<TauriPluginSvelteStoreOptions>;
  }

  public set(value: S) {
    this.store.set(value);
  }

  public update(updater: Updater<S>) {
    this.store.update(updater);
  }

  public subscribe(run: Subscriber<S>, invalidate?: () => void): Unsubscriber {
    return this.store.subscribe(run, invalidate);
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

  protected patchSelf(state: S) {
    const _state = this.applyKeyFilters(state);
    this.update((value) => Object.assign(value, _state));
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

  public async getPath() {
    return commands.getStorePath(this.id);
  }

  public async save() {
    return commands.save(this.id);
  }

  public async saveAll() {
    return commands.saveAll();
  }

  public async saveAllNow() {
    return commands.saveAllNow();
  }

  public async saveNow() {
    return commands.saveNow(this.id);
  }
}

/**
 * Create a new store with the given `id` and initial `state`.
 *
 * @example
 * ```ts
 * import { store, Store } from 'tauri-plugin-svelte';
 *
 * // These are equivalent.
 * const store = new Store('counter', { count: 0 });
 * const store = store('counter', { count: 0 });
 * ```
 */
export function store<S extends State>(
  id: string,
  state: S,
  options?: TauriPluginSvelteStoreOptions
): Store<S> {
  return new Store(id, state, options);
}
