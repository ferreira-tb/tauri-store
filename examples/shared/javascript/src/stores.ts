import type { StoreOptions } from '@tauri-store/shared';

export const counter = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
};

export const debouncedCounter = {
  debouncedCounter: 0,
  debouncedCounter2: 0,
  debouncedCounter3: 0,
};

export const throttledCounter = {
  throttledCounter: 0,
  throttledCounter2: 0,
  throttledCounter3: 0,
};

export const counterOptions: StoreOptions = {
  filterKeys: ['counter', 'counter2', 'nested'],
  filterKeysStrategy: 'pick',
  saveOnExit: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,
};

export const debouncedCounterOptions: StoreOptions = {
  filterKeys: ['debouncedCounter3'],
  filterKeysStrategy: 'omit',
  saveOnExit: true,
  saveOnChange: true,
  syncStrategy: 'debounce',
  syncInterval: 1000,
};

export const throttledCounterOptions: StoreOptions = {
  filterKeys: ['throttledCounter3'],
  filterKeysStrategy: 'omit',
  saveOnExit: false,
  saveOnChange: false,
  syncStrategy: 'throttle',
  syncInterval: 1000,
};
