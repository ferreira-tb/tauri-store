<script lang="ts">
  import type { Code } from '$lib/code';
  import { CodeBlock, CodeGroup } from '$components/code';
  import StartIt from '$content/guide/getting-started/start-it.svelte';

  const createStore: Code = {
    id: 'create-store',
    label: 'src/stores/counter.ts',
    lang: 'typescript',
    value: `
import { Store } from 'tauri-store';

const counterStore = new Store('counter', { counter: 0 });
    `,
  };

  const startStore = `
await counterStore.start();
  `;

  const useStore = `
// Get a value.
// This is a synchronous operation!
const counter = store.get('counter');
console.log(counter);

// Set a value.
store.set('counter', 42);

// Update a value with a callback.
store.update('counter', (value) => value + 1);

// Listen to changes.
store.subscribe((state) => {
  console.log(state);
});

// Save the store.
// Unlike the others, this is asynchronous.
await store.save();
`;
</script>

<li>
  <span>Create a store:</span>

  <CodeGroup code={createStore} />
</li>

<li>
  <span>Start it:</span>

  <CodeBlock lang="typescript" code={startStore} />

  <StartIt />
</li>

<li>
  <span>Use the store:</span>

  <CodeBlock lang="typescript" code={useStore} />
</li>
