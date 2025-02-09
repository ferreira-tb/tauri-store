import { snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$stores/snippet';

export const jsHooks = snippet((metadata) => {
  const name = metadata.name as TauriPlugin;
  switch (name) {
    case 'tauri-plugin-pinia': {
      return `
import { defineStore } from 'pinia';

function store() {
  return { counter: ref(0) };
}

export const useStore = defineStore('store', store, {
  tauri: {
    hooks: {
      error: (err) => console.error(err),
      beforeBackendSync: (state) => {
        console.log(state);
        return state;
      },
    }
  },
});
      `;
    }

    case 'tauri-plugin-svelte':
    case 'tauri-plugin-valtio': {
      return `
import { store } from '${name}';

const value = { counter: 0 };
const store = store('store', value, {
  hooks: {
    error: (err) => console.error(err),
    beforeBackendSync: (state) => {
      console.log(state);
      return state;
    },
  }
});
      `;
    }
  }
});

export const onBeforeBackendSync = `
const hooks: StoreHooks = {
  beforeBackendSync: (state) => {
    state.foo = null;
    state.nested.bar.baz = 'qux';
    return state;
  }
};
`;

export const onBeforeFrontendSync = `
const hooks: StoreHooks = {
  beforeFrontendSync: (state) => {
    state.foo = new Set(state.foo);
    state.bar = state.bar.map((value) => value * 2);
    return state;
  }
};
`;

export const onError = `
const hooks: StoreHooks = {
  error: (err) => console.error(err)
};
`;

export const onLoad = snippetGroup((metadata) => {
  return {
    id: 'on-load',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .on_load(|store| {
    println!("store loaded: {}", store.id());
    Ok(())
  })
  .build();
      `,
  };
});
