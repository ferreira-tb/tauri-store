<script lang="ts">
  import { NPM_SHARED } from '$lib/url';
  import { Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.10.3';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>Update <Link href={NPM_SHARED} code>@tauri-store/shared</Link>.</li>
  </ul>
</Changelog.Dependencies>
