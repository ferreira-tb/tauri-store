import type { StoreOptions } from './types';

type Options = Required<StoreOptions>;

export const DEFAULT_FILTER_KEYS: Options['filterKeys'] = null;

export const DEFAULT_FILTER_KEYS_STRATEGY: Options['filterKeysStrategy'] = 'omit';

export const DEFAULT_ON_ERROR: Options['onError'] = console.error.bind(console);

export const DEFAULT_SAVE_INTERVAL: Options['saveInterval'] = 0;

export const DEFAULT_SAVE_ON_CHANGE: Options['saveOnChange'] = false;

export const DEFAULT_SAVE_STRATEGY: Options['saveStrategy'] = 'immediate';

export const DEFAULT_SYNC_INTERVAL: Options['syncInterval'] = 0;

export const DEFAULT_SYNC_STRATEGY: Options['syncStrategy'] = 'immediate';
