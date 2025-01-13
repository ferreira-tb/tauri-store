<script lang="ts">
  import { currentMetadata } from '$stores/plugin';
  import { findByTitle } from '$lib/glob/plugin-file';

  const files = import.meta.glob(['./*.svelte', '!**/index.svelte'], { eager: true });
  const Steps = $derived.by(() => findByTitle(files, $currentMetadata.title));
</script>

<Steps />
