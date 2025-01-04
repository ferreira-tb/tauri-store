<script lang="ts">
  import * as Store from '$lib/content/changelog/tauri-store';
  import { Changelog, Container } from '$lib/components/container';
</script>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Store.V0x />
