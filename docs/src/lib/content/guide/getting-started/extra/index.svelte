<script lang="ts">
  import { findByTitle } from '$lib/glob/plugin-file';
  import { currentMetadata } from '$lib/stores/plugin';

  const files = import.meta.glob(['./*.svelte', '!**/index.svelte'], { eager: true });
  const Extra = $derived.by(() => findByTitle(files, $currentMetadata.title));
</script>

<Extra />
