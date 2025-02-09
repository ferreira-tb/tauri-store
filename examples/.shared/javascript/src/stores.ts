import type { Option, StoreOptions } from '@tauri-store/shared';

type Counter = {
  counter: number;
  counter2: Option<number>;
  counter3: Option<number>;
  nested: { foo: { bar: { baz: number } } };
  nested2: Option<{ foo: { bar: Option<{ baz: number }> } }>;
  nested3: Option<{ foo: { bar: { baz: Option<number> } } }>;
};

export const counter: Counter = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
  nested2: { foo: { bar: { baz: 0 } } },
  nested3: { foo: { bar: { baz: 0 } } },
};

export const counterOptions: StoreOptions<Counter> = {
  filterKeys: ['counter', 'nested', 'nested2', 'nested3'],
  filterKeysStrategy: 'pick',
  saveOnExit: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,

  hooks: {
    beforeBackendSync: (state) => {
      state = structuredClone(state);
      state.counter2 &&= null;
      if (state.nested2) {
        state.nested2.foo.bar &&= null;
      }

      log(state, 'beforeBackendSync');
      return state;
    },
    beforeFrontendSync: (state) => {
      state.nested3 = { foo: { bar: { baz: Date.now() } } };
      return state;
    },
  },
};

export const debouncedCounter = {
  debouncedCounter: 0,
  debouncedCounter2: 0,
  debouncedCounter3: 0,
};

export const debouncedCounterOptions: StoreOptions<typeof debouncedCounter> = {
  filterKeys: ['debouncedCounter3'],
  filterKeysStrategy: 'omit',
  saveOnExit: true,
  saveOnChange: true,
  syncStrategy: 'debounce',
  syncInterval: 1000,
};

export const throttledCounter = {
  throttledCounter: 0,
  throttledCounter2: 0,
  throttledCounter3: 0,
};

export const throttledCounterOptions: StoreOptions<typeof throttledCounter> = {
  filterKeys: ['throttledCounter3'],
  filterKeysStrategy: 'omit',
  saveOnExit: false,
  saveOnChange: false,
  syncStrategy: 'throttle',
  syncInterval: 1000,
};

function log(state: unknown, label?: string) {
  if (label) console.log(label);
  console.log(state);
  console.log('-'.repeat(20));
}
