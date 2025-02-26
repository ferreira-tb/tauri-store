<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '2.0.0';

  const pinia3 = 'https://github.com/vuejs/pinia/releases/tag/v3.0.0';
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Update <Ext href={pinia3} code>pinia</Ext> to <code>3.0</code>.
    </li>
  </ul>
</Changelog.BreakingChanges>
