import { listen, StoreEvent } from './event';
import { TimeStrategy } from './utils/time-strategy';
import type { Option, Writeable } from '@tb-dev/utils';
import type {
  ConfigChangePayload,
  PluginOptions,
  State,
  StateChangePayload,
  StoreBackendRawOptions,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
} from './types';

export abstract class BasePlugin {
  public abstract readonly id: string;
  protected abstract readonly storeOptions: StoreOptions;

  protected enabled = false;
  protected changeQueue: StateChangePayload[] = [];

  private isListeningConfigChanges = false;
  private _unlistenConfigChanges: Option<() => void>;

  public abstract start(): Promise<void>;
  public abstract stop(): Promise<void>;

  protected abstract processChangeQueue(): Promise<void>;
  protected abstract patchSelf(state: State): void;
  protected abstract patchBackend(state: State): void;

  protected abstract setStoreOptions(): Promise<void>;

  protected patchConfig(config: StoreBackendRawOptions) {
    if (typeof config.saveOnChange === 'boolean') {
      this.storeOptions.saveOnChange = config.saveOnChange;
    }

    if (Array.isArray(config.saveStrategy)) {
      const saveStrategy = TimeStrategy.parse(config.saveStrategy);
      this.storeOptions.saveInterval = saveStrategy.interval;
      this.storeOptions.saveStrategy = saveStrategy.strategy;
    }
  }

  protected async listenConfigChanges() {
    if (!this.isListeningConfigChanges) {
      this.isListeningConfigChanges = true;
      this._unlistenConfigChanges = await listen<ConfigChangePayload>(
        StoreEvent.ConfigChange,
        ({ payload }) => this.patchConfig(payload.config)
      );
    }
  }

  protected unlistenConfigChanges() {
    this.isListeningConfigChanges = false;
    this._unlistenConfigChanges?.();
    this._unlistenConfigChanges = null;
  }

  protected applyKeyFilters(state: State) {
    const keyFilter = this.storeOptions.filterKeys;
    const strategy = this.storeOptions.filterKeysStrategy ?? 'omit';

    if (keyFilter) {
      const result: State = {};
      for (const [key, value] of Object.entries(state)) {
        if (!shouldFilterKey(keyFilter, strategy, key)) {
          result[key] = value;
        }
      }

      return result;
    }

    return state;
  }

  protected get syncStrategy() {
    return this.storeOptions.syncStrategy;
  }

  protected get syncInterval() {
    return this.storeOptions.syncInterval;
  }

  protected get onError() {
    return this.storeOptions.onError;
  }
}

/**
 * @internal
 */
export function mergePluginOptions<T extends PluginOptions, U extends PluginOptions>(
  target: T = {} as T,
  source: U = {} as U
): T & U {
  type Key = keyof PluginOptions;
  for (const [key, value] of Object.entries(source)) {
    if (typeof target[key as Key] === 'undefined') {
      (target as Writeable<PluginOptions>)[key as Key] = value;
    }
  }

  return target as T & U;
}

function shouldFilterKey(
  keyFilter: StoreKeyFilter,
  strategy: StoreKeyFilterStrategy,
  key: string
): boolean {
  return (
    (strategy === 'omit' && isStoreKeyMatch(keyFilter, key)) ||
    (strategy === 'pick' && !isStoreKeyMatch(keyFilter, key)) ||
    (typeof strategy === 'function' && !strategy(key))
  );
}

function isStoreKeyMatch(keyFilter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof keyFilter === 'string' && key === keyFilter) ||
    (Array.isArray(keyFilter) && keyFilter.includes(key)) ||
    (keyFilter instanceof RegExp && keyFilter.test(key))
  );
}
