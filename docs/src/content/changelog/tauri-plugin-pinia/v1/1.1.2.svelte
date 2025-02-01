<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '1.1.2';

  const url = 'https://docs.rs/tauri-store/0.7.1/tauri_store/';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>
      Update <Ext href={url} code>tauri-store</Ext> to
      <Link href="/tauri-store/changelog/tauri-store#v0-7-1" code>0.7.1</Link>.
    </li>
  </ul>
</Changelog.Dependencies>
