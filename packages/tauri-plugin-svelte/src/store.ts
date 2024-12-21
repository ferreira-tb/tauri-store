import { tick } from 'svelte';
import * as commands from './commands';
import type { TauriPluginSvelteStoreOptions } from './types';
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
  type State,
  type TauriStoreContract,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

/**
 * Implementing `Writable<S>` ensures that the store also adheres to the Svelte store contract.
 *
 * @see https://svelte.dev/docs/svelte/stores#Store-contract
 */
export type StoreContract<S extends State> = TauriStoreContract & Writable<S>;

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
  protected override options: TauriPluginSvelteStoreOptions;
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
      onError: options.onError ?? console.error,
      saveOnChange: options.saveOnChange ?? false,
      saveInterval: saveStrategy.interval,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    };
  }

  public set(value: S) {
    this.store.set(value);
  }

  public update(updater: Updater<S>): void {
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

  protected override watch() {
    const patchBackend = (value: S) => this.patchBackend(value);
    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return this.store.subscribe(fn);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return this.store.subscribe(fn);
    }

    return this.store.subscribe(patchBackend);
  }

  protected override patchSelf(state: S) {
    const _state = this.applyKeyFilters(state);
    this.store.update((value) => Object.assign(value, _state));
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
