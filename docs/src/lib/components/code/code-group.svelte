<script lang="ts">
  import * as Tabs from '../ui/tabs';
  import type { Code } from '$lib/code';
  import CodeBlock from './code-block.svelte';
  import * as Accordion from '$lib/components/ui/accordion';

  interface Props {
    accordion?: boolean | string;
    code: Code | Code[];
  }

  const { code, accordion }: Props = $props();

  const group = $derived.by(() => {
    return Array.isArray(code) ? code : [code];
  });

  let activeTab = $state(defaultTab());

  const trigger = $derived.by(() => {
    return typeof accordion === 'string' ? accordion : 'Show code';
  });

  function defaultTab() {
    const first = group.at(0);
    return first?.id ?? first?.label;
  }
</script>

{#snippet codeGroup()}
  <div class="ts-code-group">
    <Tabs.Root bind:value={activeTab} class="relative mx-0 w-full">
      <div class="flex items-center justify-between">
        <Tabs.List class=" w-full justify-start rounded-none border-b bg-transparent p-0">
          {#each group as { id, label } (id ?? label)}
            <Tabs.Trigger
              value={id ?? label}
              class="text-muted-foreground data-[state=active]:border-b-primary data-[state=active]:text-foreground relative h-9 rounded-none border-b-2 border-b-transparent bg-transparent px-4 pb-3 pt-2 font-semibold shadow-none transition-none data-[state=active]:shadow-none"
            >
              {label}
            </Tabs.Trigger>
          {/each}
        </Tabs.List>
      </div>
      {#each group as { id, label, value, lang } (id ?? label)}
        <Tabs.Content value={id ?? label} class="relative mt-0">
          <CodeBlock code={value} {lang} accordion={false} class="rounded-t-none" />
        </Tabs.Content>
      {/each}
    </Tabs.Root>
  </div>
{/snippet}

{#if accordion}
  <Accordion.Root type="single">
    <Accordion.Item class="border-b-0">
      <Accordion.Trigger>{trigger}</Accordion.Trigger>
      <Accordion.Content>
        {@render codeGroup()}
      </Accordion.Content>
    </Accordion.Item>
  </Accordion.Root>
{:else}
  {@render codeGroup()}
{/if}

<style>
  :global(.ts-mobile .ts-code-group [data-tabs-list]) {
    margin: 0 calc(var(--content-padding) * -1);
    width: 100vw;
  }

  .ts-code-group :global(pre.shiki) {
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
  }

  .ts-code-group:not(:first-child) {
    margin-top: 1.5rem;
  }

  .ts-code-group:not(:last-child) {
    margin-bottom: 1.5rem;
  }
</style>
