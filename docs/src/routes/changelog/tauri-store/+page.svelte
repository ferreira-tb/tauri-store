<script lang="ts">
  import * as Store from '$lib/content/changelog/tauri-store';
  import { Changelog, Container } from '$lib/components/container';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-store" />
</svelte:head>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Store.V0x />
