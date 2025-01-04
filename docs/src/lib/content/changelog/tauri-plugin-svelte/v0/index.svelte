<script lang="ts">
  import { sortVersions } from '$lib/content/changelog/version';

  const versions = import.meta.glob('./0.*.svelte', { eager: true });
  const components = sortVersions(versions);
</script>

{#each components as [version, component] (version)}
  <svelte:component this={component} />
{/each}
