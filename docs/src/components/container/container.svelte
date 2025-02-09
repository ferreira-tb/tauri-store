<script lang="ts">
  import type { Snippet } from 'svelte';
  import { kebabCase } from 'change-case';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    children?: Snippet;
    level?: 1 | 2 | 3 | 4 | 5 | 6;
    title: string;
    titleSnippet?: Snippet<[{ title: string }]>;
  };

  const { id, title, level = 2, titleSnippet, children, ...restProps }: Props = $props();
</script>

<article {...restProps}>
  <svelte:element this={`h${level}`} id={id ?? kebabCase(title)} tabindex="-1">
    {#if titleSnippet}
      {@render titleSnippet({ title })}
    {:else}
      {title}
    {/if}
  </svelte:element>

  {#if children}
    <div>
      {@render children()}
    </div>
  {/if}
</article>
