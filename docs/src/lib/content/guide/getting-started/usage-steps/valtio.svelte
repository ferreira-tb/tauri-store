<script lang="ts">
  import type { Code } from '$lib/code';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import StartIt from '$lib/content/guide/getting-started/start-it.svelte';

  const createStore: Code = {
    id: 'create-store',
    label: 'src/stores/counter.ts',
    lang: 'typescript',
    value: `
import { store } from 'tauri-plugin-valtio';

export const counterStore = store('counter', { counter: 0 });

export const increment = () => {
  counterStore.state.counter++;
};
    `,
  };

  const startStore = `
import { counterStore } from '@/stores/counter';

await counterStore.start();
  `;

  const useStore = `
import { useSnapshot } from 'valtio';
import { counterStore } from '@/stores/counter';

export default function MyComponent() {
  // The \`state\` property is the actual valtio proxy.
  const snap = useSnapshot(fooStore.state);

  return (
    <div>
      <p>Counter: {snap.counter}</p>
      <button type="button" onClick={fooStore.increment}>
        <span>Increment</span>
      </button>
    </div>
  );
}
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
  <span>Use the store in your React components:</span>

  <CodeBlock lang="tsx" code={useStore} />
</li>
