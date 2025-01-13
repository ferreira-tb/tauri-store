<script lang="ts">
  import { Changelog } from '$components/container';

  const version = '0.10.2';
</script>

<Changelog.Version {version} />

<Changelog.BugFixes {version}>
  <ul>
    <li>Fix the filter for store keys.</li>
  </ul>
</Changelog.BugFixes>
