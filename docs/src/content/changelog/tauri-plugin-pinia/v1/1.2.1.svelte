<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '1.2.1';

  const url = 'https://docs.rs/tauri-store/0.8.0/tauri_store/';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>
      Update <Ext href={url} code>tauri-store</Ext> to
      <Link href="/tauri-store/changelog/tauri-store#v0-8-0" code>0.8.0</Link>.
    </li>
  </ul>
</Changelog.Dependencies>
