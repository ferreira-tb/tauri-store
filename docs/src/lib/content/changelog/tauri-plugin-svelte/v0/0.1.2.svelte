<script lang="ts">
  import { Changelog } from '$lib/components/container';

  const version = '0.1.2';
</script>

<Changelog.Version {version} />

<Changelog.BugFixes {version}>
  <ul>
    <li>Set <code>svelte</code> as an external dependency.</li>
  </ul>
</Changelog.BugFixes>
