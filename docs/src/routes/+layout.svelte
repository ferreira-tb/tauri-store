<script lang="ts">
  import '../app.css';
  import { ModeWatcher } from 'mode-watcher';
  import metadata from '$lib/data/metadata.json';
  import { Layout } from '$lib/components/layout';
  import { currentPlugin } from '$lib/stores/plugin';
  import { onMount, type Snippet, tick } from 'svelte';
  import * as Sidebar from '$lib/components/layout/sidebar';

  const { children }: { children: Snippet } = $props();

  onMount(async () => {
    await tick();
    const search = new URLSearchParams(location.search);
    const plugin = search.get('plugin');
    if (plugin) {
      const data = metadata.find((item) => item.name === plugin);
      if (data?.isPlugin) currentPlugin.set(data.name);
    }
  });
</script>

<Sidebar.Provider>
  <Sidebar.Root />
  <ModeWatcher defaultMode="dark" />

  <Layout>
    {@render children()}
  </Layout>
</Sidebar.Provider>
