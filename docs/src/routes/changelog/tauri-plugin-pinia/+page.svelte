<script lang="ts">
  import { Changelog, Container } from '$lib/components/container';
  import * as Pinia from '$lib/content/changelog/tauri-plugin-pinia';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-pinia" />
</svelte:head>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Pinia.V0x />
