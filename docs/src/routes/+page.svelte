<script lang="ts">
  import { findMetadata } from '$lib/data';
  import Link from '$lib/components/link.svelte';
  import * as Alert from '$lib/components/alert';
  import Container from '$lib/components/container.svelte';
  import PluginLink from '$lib/components/plugin-link.svelte';

  const url = {
    cargoFeatures: 'https://doc.rust-lang.org/cargo/reference/features.html',
    discord: 'https://discord.gg/ARd7McmVNv',
    discussions: 'https://github.com/ferreira-tb/tauri-store/discussions',
    fileSyncAll: 'https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all',
    semver: 'https://doc.rust-lang.org/cargo/reference/semver.html',
    tauriStore: findMetadata('tauri-store').docs.rust,
  };
</script>

<div>
  <Container title="tauri-store" id="tauri-store" level={1}>
    <p>Persistent stores for Tauri.</p>
  </Container>

  <Container title="Features" id="features" level={2}>
    <ul>
      <li>Save your stores to disk.</li>
      <li>Synchronize across multiple windows.</li>
      <li>Debounce or throttle store updates.</li>
      <li>Access the stores from both JavaScript and Rust.</li>
    </ul>
  </Container>

  <Container title="Framework support" id="framework-support" level={2}>
    <p>
      The <Link href={url.tauriStore} external><code>tauri-store</code></Link> crate is a framework-agnostic
      backend for store plugins. Currently, the following plugins are available:
    </p>
    <ul>
      {#snippet pluginLink(label: string, name: TauriPlugin)}
        <li>
          {label}: <PluginLink plugin={name}>{name}</PluginLink>
        </li>
      {/snippet}

      {@render pluginLink('Pinia', 'tauri-plugin-pinia')}
      {@render pluginLink('Svelte', 'tauri-plugin-svelte')}
    </ul>
  </Container>

  <Container title="Optional features" id="optional-features" level={2}>
    <ul>
      <li>
        <code>file-sync-all</code>: Calls
        <Link href={url.fileSyncAll} external><code>File::sync_all</code></Link>
        after writing to the store file to ensure that all in-memory data reaches the filesystem. Enabling
        this can significantly degrade performance.
      </li>
    </ul>
  </Container>

  <Container title="Versioning" id="versioning" level={2}>
    <p>
      This crate follows
      <Link href={url.semver} external>Cargo guidelines for SemVer compatibility</Link>.
    </p>

    <Alert.Root>
      <Alert.Title>Experimental features</Alert.Title>
      <Alert.Description>
        <Link href={url.cargoFeatures} external>Cargo features</Link> prefixed with
        <code>unstable-</code> are experimental and may introduce breaking changes between patch versions
        or even be completely removed.
      </Alert.Description>
    </Alert.Root>
  </Container>

  <Container title="Any questions?" id="any-questions" level={2}>
    <p>
      Feel free to start a discussion on the
      <Link href={url.discussions} external>GitHub repository</Link>
      or ask in our <Link href={url.discord} external>Discord server</Link>.
    </p>
  </Container>
</div>
