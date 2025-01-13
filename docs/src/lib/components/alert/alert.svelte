<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import { Root } from '$components/ui/alert';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    children: Snippet;
  };

  const { class: className, children }: Props = $props();
</script>

<div class={cn('[&:not(:first-child)]:pt-4', className)}>
  <Root>
    {@render children()}
  </Root>
</div>
