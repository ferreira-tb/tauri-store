<script lang="ts">
  import data from './data/0.10.0.json';
  import type { Code } from '$lib/code';
  import { Ext } from '$lib/components/link';
  import { CodeGroup } from '$lib/components/code';
  import { Changelog } from '$lib/components/container';

  const version = '0.10.0';

  const code: Code = {
    id: 'custom-path',
    lang: 'rust',
    label: 'src-tauri/src/main.rs',
    value: `
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // The old default path.
      let path = app.path().app_data_dir()?.join("pinia");
      app.app_handle().plugin(
        tauri_plugin_pinia::Builder::new()
          .path(path)
          .build(),
      )?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
    `,
  };
</script>

{#snippet ext(key: keyof typeof data.url, label?: string)}
  <Ext href={data.url[key]} code>{label ?? key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Update {@render ext('tauri-store')} to <code>0.5.0</code>.</li>
    <li>
      Change the default directory name for stores from <code>pinia</code> to
      <code>tauri-plugin-pinia</code>. If you’re using a {@render ext('path', 'custom path')}, this
      change won’t affect you. Otherwise, you’ll need to either move your existing stores to the new
      default directory or manually set the path to match the previous configuration.

      <CodeGroup {code} accordion="Show code example" />
    </li>
  </ul>
</Changelog.BreakingChanges>
