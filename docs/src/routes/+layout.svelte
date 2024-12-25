<script lang="ts">
  import '../app.css';
  import type { Snippet } from 'svelte';
  import { ModeWatcher } from 'mode-watcher';
  import Sidebar from '$lib/components/sidebar.svelte';
  import SidebarProvider from '$lib/components/ui/sidebar/sidebar-provider.svelte';

  const { children }: { children: Snippet } = $props();
</script>

<SidebarProvider>
  <Sidebar />
  <ModeWatcher defaultMode="dark" />

  <main class="flex-1 p-4">
    {@render children()}
  </main>
</SidebarProvider>
