<script lang="ts">
  import { Changelog } from '$components/container';

  const version = '0.7.1';
</script>

<Changelog.Version {version} />

<Changelog.BugFixes {version}>
  <ul>
    <li>Remove <code>#[expect]</code> attribute.</li>
  </ul>
</Changelog.BugFixes>
