<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
    class?: string;
    id: string;
    level: 1 | 2 | 3 | 4 | 5 | 6;
    title: string;
  }

  const p: Props = $props();

  const tag = $derived(`h${p.level}`);
</script>

<div class={cn(p.class)}>
  <svelte:element this={tag} id={p.id} tabindex="-1">
    {p.title}
  </svelte:element>

  <div class="mt-6">
    {@render p.children()}
  </div>
</div>
