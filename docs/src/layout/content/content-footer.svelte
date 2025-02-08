<script lang="ts">
  import { cn } from '$lib/utils';
  import { LICENSE } from '$lib/url';
  import { Ext } from '$components/link';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    class?: string;
  };

  const { class: className }: Props = $props();
</script>

<div class={cn('text-muted-foreground my-12 w-full text-center text-xs', className)}>
  <Ext href={LICENSE}>
    <span>MIT © 2025 Andrew Ferreira</span>
  </Ext>
</div>
