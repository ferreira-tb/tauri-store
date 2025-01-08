<script lang="ts">
  import { cn } from '$lib/utils';
  import { highlight, type Lang } from '$lib/code';
  import type { HTMLAttributes } from 'svelte/elements';
  import * as Accordion from '$lib/components/ui/accordion';

  type Props = HTMLAttributes<HTMLElement> & {
    accordion?: boolean | string;
    class?: string;
    code: string | null;
    lang: Lang;
  };

  const { code, lang, accordion, class: className, ...restProps }: Props = $props();

  const trigger = $derived.by(() => {
    return typeof accordion === 'string' ? accordion : 'Show code';
  });
</script>

{#snippet codeBlock()}
  <div {...restProps} class={cn('ts-code-block', className)}>
    {#if code}
      {#await highlight(code.trim(), { lang }) then value}
        {@html value}
      {/await}
    {/if}
  </div>
{/snippet}

{#if accordion}
  <Accordion.Root type="single">
    <Accordion.Item class="border-b-0">
      <Accordion.Trigger>{trigger}</Accordion.Trigger>
      <Accordion.Content>
        {@render codeBlock()}
      </Accordion.Content>
    </Accordion.Item>
  </Accordion.Root>
{:else}
  {@render codeBlock()}
{/if}

<style>
  :global(.ts-mobile .ts-code-block) {
    margin: 0 calc(var(--content-padding) * -1);
  }

  .ts-code-block > :global(pre) {
    position: relative;
    z-index: 1;
    margin: 0;
    border-radius: 0.5rem;
    padding: 1rem;
    overflow-x: auto;
  }

  :global(.ts-mobile) .ts-code-block > :global(pre) {
    border-radius: 0;
  }

  .ts-code-block :global(code) {
    font-size: 0.875rem;
    line-height: 1.7;
    hyphens: none;
    direction: ltr;
  }

  .ts-code-block:not(:first-child) {
    margin-top: 1.5rem;
  }

  .ts-code-block:not(:last-child) {
    margin-bottom: 1.5rem;
  }
</style>
