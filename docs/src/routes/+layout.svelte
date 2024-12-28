<script lang="ts">
  import '../app.css';
  import type { Snippet } from 'svelte';
  import { ModeWatcher } from 'mode-watcher';
  import { Layout } from '$lib/components/layout';
  import * as Sidebar from '$lib/components/sidebar';

  const { children }: { children: Snippet } = $props();
</script>

<Sidebar.Provider>
  <Sidebar.Root />
  <ModeWatcher defaultMode="dark" />

  <Layout>
    {@render children()}
  </Layout>
</Sidebar.Provider>
