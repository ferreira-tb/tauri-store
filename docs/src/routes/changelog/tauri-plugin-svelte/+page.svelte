<script lang="ts">
  import { Changelog, Container } from '$lib/components/container';
  import * as Svelte from '$lib/content/changelog/tauri-plugin-svelte';
</script>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Svelte.V0x />
