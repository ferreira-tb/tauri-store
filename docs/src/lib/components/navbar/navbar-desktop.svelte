<script lang="ts">
  import * as Icon from '../icons';
  import Link from '../link.svelte';
  import NavbarMenu from './navbar-menu.svelte';
  import { Separator } from '$lib/components/ui/separator';
  import { changelogs, javascriptDocs, rustDocs } from '$lib/data';

  const url = {
    discord: 'https://discord.gg/ARd7McmVNv',
    github: 'https://github.com/ferreira-tb/tauri-store',
  };
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

<div class="flex items-center justify-end gap-4">
  <Link href={url.github} external>
    <Icon.Github size="1.25rem" />
  </Link>
  <Link href={url.discord} external>
    <Icon.Discord size="1.25rem" />
  </Link>
</div>
