<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Root } from '$lib/components/ui/alert';

  interface Props {
    children: Snippet;
  }

  const { children }: Props = $props();
</script>

<div class="[&:not(:first-child)]:pt-4">
  <Root>
    {@render children()}
  </Root>
</div>
