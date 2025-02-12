import { snakeCase } from 'change-case';
import { snippet } from '$stores/snippet';

export const get = snippet((metadata, ctx) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn get_counter(app: AppHandle) -> i32 {
  let value = app
    .${ctx.collection}()
    .get("store", "counter")
    .unwrap();

  serde_json::from_value(value).unwrap()
}
  `;
});

export const tryGet = snippet((metadata, ctx) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn try_get_counter(app: AppHandle) -> i32 {
  app
    .${ctx.collection}()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
  `;
});

export const watchStore = snippet((metadata, ctx) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn watch_store(app: AppHandle) {
  let id = app.${ctx.collection}().watch("store", |app| {
    app
      .${ctx.collection}()
      .try_get::<i32>("store", "counter")
      .inspect(|counter| println!("counter: {counter}"))?;
    
    Ok(())
  });

  // It returns an id that can be used to remove the watcher.
  if let Ok(id) = id {
    app.${ctx.collection}().unwatch("store", id).unwrap();
  }
}
  `;
});
