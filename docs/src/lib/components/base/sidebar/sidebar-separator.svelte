<script lang="ts">
  import { cn } from '$lib/utils.js';
  import type { ComponentProps } from 'svelte';
  import { Separator } from '$components/base/separator/index.js';

  let {
    ref = $bindable(null),
    class: className,
    ...restProps
  }: ComponentProps<typeof Separator> = $props();
</script>

<Separator
  bind:ref
  data-sidebar="separator"
  class={cn('bg-sidebar-border mx-2 w-auto', className)}
  {...restProps}
/>
