<script lang="ts">
  import type { Snippet } from 'svelte';
  import { kebabCase } from 'change-case';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    children?: Snippet;
    class?: string;
    id?: string;
    level?: 1 | 2 | 3 | 4 | 5 | 6;
    title: string;
  };

  const { id, level = 2, title, children, ...restProps }: Props = $props();
</script>

<article {...restProps}>
  <svelte:element this={`h${level}`} id={id ?? kebabCase(title)} tabindex="-1">
    {title}
  </svelte:element>

  {#if children}
    <div class="mt-6">
      {@render children()}
    </div>
  {/if}
</article>
