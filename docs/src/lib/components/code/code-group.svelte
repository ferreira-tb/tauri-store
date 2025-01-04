<script lang="ts">
  import * as Tabs from '../ui/tabs';
  import type { Code } from '$lib/code';
  import CodeBlock from './code-block.svelte';

  interface Props {
    code: Code[];
  }

  const { code }: Props = $props();

  let activeTab = $state(defaultTab());

  function defaultTab() {
    const first = code.at(0);
    return first?.id ?? first?.label;
  }
</script>

<div class="ts-code-group">
  {#if code.length > 0}
    <Tabs.Root bind:value={activeTab} class="relative mr-auto w-full">
      <div class="flex items-center justify-between">
        <Tabs.List class="w-full justify-start rounded-none border-b bg-transparent p-0">
          {#each code as { id, label } (id ?? label)}
            <Tabs.Trigger
              value={id ?? label}
              class="text-muted-foreground data-[state=active]:border-b-primary data-[state=active]:text-foreground relative h-9 rounded-none border-b-2 border-b-transparent bg-transparent px-4 pb-3 pt-2 font-semibold shadow-none transition-none data-[state=active]:shadow-none"
            >
              {label}
            </Tabs.Trigger>
          {/each}
        </Tabs.List>
      </div>
      {#each code as { id, label, value, lang } (id ?? label)}
        <Tabs.Content value={id ?? label} class="relative mt-1 rounded-md border">
          <CodeBlock code={value} {lang} accordion={false} />
        </Tabs.Content>
      {/each}
    </Tabs.Root>
  {/if}
</div>

<style>
  .ts-code-group:not(:first-child) {
    margin-top: 1.5rem;
  }

  .ts-code-group:not(:last-child) {
    margin-bottom: 1.5rem;
  }
</style>
