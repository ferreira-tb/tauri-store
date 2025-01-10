<script lang="ts">
  import { cn } from '$lib/utils';
  import { Ext } from '$lib/components/link';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    class?: string;
  };

  const { class: className }: Props = $props();
</script>

<div class={cn('text-muted-foreground mt-12 w-full text-center text-xs', className)}>
  <Ext href="https://github.com/ferreira-tb/tauri-store/blob/main/LICENSE">
    <span>MIT © 2025 Andrew Ferreira</span>
  </Ext>
</div>
