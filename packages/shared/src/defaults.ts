import type { StoreOptions } from './types';

export const DEFAULT_FILTER_KEYS: Required<StoreOptions>['filterKeys'] = null;

export const DEFAULT_FILTER_KEYS_STRATEGY: Required<StoreOptions>['filterKeysStrategy'] = 'omit';

export const DEFAULT_ON_ERROR: Required<StoreOptions>['onError'] = console.error.bind(console);

export const DEFAULT_SAVE_INTERVAL: Required<StoreOptions>['saveInterval'] = 0;

export const DEFAULT_SAVE_ON_CHANGE: Required<StoreOptions>['saveOnChange'] = false;

export const DEFAULT_SAVE_STRATEGY: Required<StoreOptions>['saveStrategy'] = 'immediate';

export const DEFAULT_SYNC_INTERVAL: Required<StoreOptions>['syncInterval'] = 0;

export const DEFAULT_SYNC_STRATEGY: Required<StoreOptions>['syncStrategy'] = 'immediate';