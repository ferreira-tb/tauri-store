<script lang="ts">
  import { Breadcrumb } from '$components/breadcrumb';
  import { Changelog, Container } from '$components/container';
  import * as Valtio from '$content/changelog/tauri-plugin-valtio';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-valtio" />
</svelte:head>

<Breadcrumb current="tauri-plugin-valtio" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next></Changelog.Next>

<Valtio.V2x />
<Valtio.V1x />
<Valtio.V0x />
