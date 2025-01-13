<script lang="ts">
  import { Changelog } from '$components/container';

  const version = '0.1.3';
</script>

<Changelog.Version {version} />

<Changelog.BugFixes {version}>
  <ul>
    <li>Fix the filter for store keys.</li>
  </ul>
</Changelog.BugFixes>
