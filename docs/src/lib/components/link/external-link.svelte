<script lang="ts">
  import { default as Link, type LinkProps } from './link.svelte';

  const linkProps: Omit<LinkProps, 'external'> = $props();
</script>

<Link {...linkProps} external />
