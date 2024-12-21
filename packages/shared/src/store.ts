import { listen, StoreEvent } from './event';
import { TimeStrategy } from './utils/time-strategy';
import type { Option, Writeable } from '@tb-dev/utils';
import type {
  ConfigChangePayload,
  State,
  StateChangePayload,
  StoreBackendRawOptions,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
} from './types';

export abstract class BaseStore {
  public abstract readonly id: string;
  protected abstract readonly options: StoreOptions;

  protected enabled = false;
  protected changeQueue: StateChangePayload[] = [];

  private isListeningConfigChanges = false;
  private removeConfigListener: Option<() => void> = null;

  public abstract start(): Promise<void>;
  public abstract stop(): Promise<void>;

  protected abstract processChangeQueue(): Promise<void>;
  protected abstract patchSelf(state: State): void;
  protected abstract patchBackend(state: State): void;

  protected abstract setOptions(): Promise<void>;

  protected patchConfig(config: StoreBackendRawOptions) {
    if (typeof config.saveOnChange === 'boolean') {
      this.options.saveOnChange = config.saveOnChange;
    }

    if (Array.isArray(config.saveStrategy)) {
      const saveStrategy = TimeStrategy.parse(config.saveStrategy);
      this.options.saveInterval = saveStrategy.interval;
      this.options.saveStrategy = saveStrategy.strategy;
    }
  }

  protected async listenConfigChanges() {
    if (!this.isListeningConfigChanges) {
      this.isListeningConfigChanges = true;
      try {
        this.removeConfigListener = await listen<ConfigChangePayload>(
          StoreEvent.ConfigChange,
          ({ payload }) => this.patchConfig(payload.config)
        );
      } catch (err) {
        this.isListeningConfigChanges = false;
        this.onError?.(err);
      }
    }
  }

  protected unlistenConfigChanges() {
    this.removeConfigListener?.();
    this.removeConfigListener = null;
    this.isListeningConfigChanges = false;
  }

  protected applyKeyFilters(state: State) {
    if (!this.options.filterKeys) {
      return state;
    }

    const result: State = {};
    const filter = this.options.filterKeys;
    const strategy = this.options.filterKeysStrategy ?? 'omit';

    for (const [key, value] of Object.entries(state)) {
      if (!shouldFilterKey(filter, strategy, key)) {
        result[key] = value;
      }
    }

    return result;
  }

  protected get syncStrategy() {
    return this.options.syncStrategy;
  }

  protected get syncInterval() {
    return this.options.syncInterval;
  }

  protected get onError() {
    return this.options.onError;
  }
}

/**
 * @internal
 */
export function mergeStoreOptions<T extends StoreOptions, U extends StoreOptions>(
  target: T = {} as T,
  source: U = {} as U
): T & U {
  type Key = keyof StoreOptions;
  for (const [key, value] of Object.entries(source)) {
    if (typeof target[key as Key] === 'undefined') {
      (target as Writeable<StoreOptions>)[key as Key] = value;
    }
  }

  return target as T & U;
}

function shouldFilterKey(
  filter: StoreKeyFilter,
  strategy: StoreKeyFilterStrategy,
  key: string
): boolean {
  return (
    (strategy === 'omit' && isStoreKeyMatch(filter, key)) ||
    (strategy === 'pick' && !isStoreKeyMatch(filter, key)) ||
    (typeof strategy === 'function' && !strategy(key))
  );
}

function isStoreKeyMatch(filter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof filter === 'string' && key === filter) ||
    (Array.isArray(filter) && filter.includes(key)) ||
    (filter instanceof RegExp && filter.test(key))
  );
}
