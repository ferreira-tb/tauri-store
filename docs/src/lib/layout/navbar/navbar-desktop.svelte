<script lang="ts">
  import NavbarMenu from './navbar-menu.svelte';
  import NavbarSocial from './navbar-social.svelte';
  import { Separator } from '$components/ui/separator';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';
</script>

<nav class="flex items-center gap-6 font-semibold">
  <a href="/tauri-store/guide/getting-started">Guide</a>
  <NavbarMenu label="JavaScript" items={javascriptDocs} external />
  <NavbarMenu label="Rust" items={rustDocs} external />
  <NavbarMenu label="Changelog" items={changelogs} />
</nav>

<div class="h-[calc(100%-2rem)]">
  <Separator orientation="vertical" class="mx-4" />
</div>

<NavbarSocial />
