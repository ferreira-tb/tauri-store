<script lang="ts">
  import { Breadcrumb } from '$components/breadcrumb';
  import { Changelog, Container } from '$components/container';
  import * as Pinia from '$content/changelog/tauri-plugin-pinia';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-pinia" />
</svelte:head>

<Breadcrumb current="tauri-plugin-pinia" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next></Changelog.Next>

<Pinia.V3x />
<Pinia.V2x />
<Pinia.V1x />
<Pinia.V0x />
