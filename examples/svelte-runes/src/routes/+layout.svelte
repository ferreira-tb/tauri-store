<script lang="ts">
  import '../../../.shared/assets/style.css';
  import type { Snippet } from 'svelte';

  const { children }: { children: Snippet } = $props();
</script>

<main>
  {@render children()}
</main>
