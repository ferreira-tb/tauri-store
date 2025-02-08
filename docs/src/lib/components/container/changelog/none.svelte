<script lang="ts">
  import Version from './version.svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  type Props = HTMLAttributes<HTMLElement> & {
    class?: string;
    version: string;
  };

  const { version, ...restProps }: Props = $props();
</script>

<Version {...restProps} {version}>No significant changes.</Version>
