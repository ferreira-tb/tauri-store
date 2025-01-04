<script lang="ts">
  import { Changelog, Container } from '$lib/components/container';
  import * as Pinia from '$lib/content/changelog/tauri-plugin-pinia';
</script>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Pinia.V0x />
