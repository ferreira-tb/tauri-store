<script lang="ts">
  import { findMetadata } from '$lib/data';
  import { DISCORD, ISSUES } from '$lib/url';
  import { useSidebar } from '$layout/sidebar';
  import metadata from '$lib/data/metadata.json';
  import * as Table from '$components/base/table';
  import { Container } from '$components/container';
  import { Ext, PluginLink } from '$components/link';
  import { resolveIcon, worksWith } from '$lib/plugin';

  const sidebar = useSidebar();

  const url = {
    cargoFeatures: 'https://doc.rust-lang.org/cargo/reference/features.html',
    fileSyncAll: 'https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all',
    semver: 'https://doc.rust-lang.org/cargo/reference/semver.html',
    tauriStore: findMetadata('tauri-store').docs.rust,
  };
</script>

<svelte:head>
  <title>tauri-store</title>
  <meta name="description" content="Persistent stores for Tauri" />
</svelte:head>

<div>
  <Container title="tauri-store" level={1}>
    <p>Persistent stores for Tauri.</p>
  </Container>

  <Container title="Features">
    <ul>
      <li>Save your stores to disk.</li>
      <li>Synchronize across multiple windows.</li>
      <li>Debounce or throttle store updates.</li>
      <li>Access the stores from both JavaScript and Rust.</li>
    </ul>
  </Container>

  <Container title="Framework support">
    <p>
      The <Ext href={url.tauriStore} code>tauri-store</Ext> crate is a framework-agnostic backend for
      store plugins. Currently, the following plugins are available:
    </p>

    <Table.Root class="mt-4">
      <Table.Header>
        <Table.Row>
          <Table.Cell>Plugin</Table.Cell>
          <Table.Cell>Version</Table.Cell>

          {#if !sidebar.isMobile}
            <Table.Cell>Downloads</Table.Cell>
            <Table.Cell>Works with</Table.Cell>
          {/if}
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each metadata as plugin (plugin.name)}
          {@const Icon = resolveIcon(plugin.name as TauriPlugin)}
          <Table.Row>
            <Table.Cell>
              <PluginLink
                plugin={plugin.name as TauriPlugin}
                class="flex items-center justify-start gap-1"
              >
                <Icon size="1.25em" />
                <span>{plugin.name}</span>
              </PluginLink>
            </Table.Cell>

            <Table.Cell>
              <Ext href={`https://www.npmjs.com/package/${plugin.name}`}>
                <img
                  src={`https://img.shields.io/npm/v/${plugin.name}`}
                  alt={plugin.name}
                  fetchpriority="low"
                  decoding="async"
                  loading="lazy"
                />
              </Ext>
            </Table.Cell>

            {#if !sidebar.isMobile}
              <Table.Cell>
                <Ext href={`https://crates.io/crates/${plugin.name}`}>
                  <img
                    src={`https://img.shields.io/crates/d/${plugin.name}`}
                    alt={plugin.name}
                    fetchpriority="low"
                    decoding="async"
                    loading="lazy"
                  />
                </Ext>
              </Table.Cell>

              <Table.Cell>
                {worksWith(plugin.name as TauriPlugin).join(', ')}
              </Table.Cell>
            {/if}
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </Container>

  <Container title="Cargo features">
    <p>
      You can enable some <Ext href={url.cargoFeatures}>Cargo features</Ext>
      to customize the plugin's behavior.
    </p>

    <ul class="mx-0 list-inside">
      <li>
        <strong>file-sync-all</strong>: calls <Ext href={url.fileSyncAll} code>File::sync_all</Ext>
        after writing to the store file to ensure that all in-memory data reaches the filesystem. Enabling
        this can significantly degrade performance.
      </li>
    </ul>
  </Container>

  <Container title="Versioning">
    <p>
      This crate follows
      <Ext href={url.semver}>Cargo guidelines for SemVer compatibility</Ext>. However, features
      prefixed with <code>unstable</code> are experimental and may introduce breaking changes between
      minor versions or even be completely removed.
    </p>
  </Container>

  <Container title="Any questions?" id="any-questions">
    <p>
      Feel free to open an issue on the
      <Ext href={ISSUES}>GitHub repository</Ext>
      or ask in our <Ext href={DISCORD}>Discord server</Ext>.
    </p>
  </Container>
</div>
