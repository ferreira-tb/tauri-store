<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '1.0.0';

  const tauriStore = 'https://docs.rs/tauri-store/0.6.0/tauri_store/';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <p>This change only affects Rust users.</p>

  <ul>
    <li>
      Update <Ext href={tauriStore} code>tauri-store</Ext> to
      <Link href="/tauri-store/changelog/tauri-store#v0-6-0" code>0.6.0</Link>.
    </li>
  </ul>
</Changelog.BreakingChanges>
