import { snakeCase } from 'change-case';
import { snippet } from '$stores/snippet';

export const get = snippet((metadata) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn get_counter(app: AppHandle) -> i32 {
  let value = app
    .${snakeCase(metadata.title!)}()
    .get("store", "counter")
    .unwrap();

  serde_json::from_value(value).unwrap()
}
  `;
});

export const tryGet = snippet((metadata) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn try_get_counter(app: AppHandle) -> i32 {
  app
    .${snakeCase(metadata.title!)}()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
  `;
});

export const watchStore = snippet((metadata) => {
  const title = snakeCase(metadata.title!);
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
fn watch_store(app: AppHandle) {
  let id = app.${title}().watch("store", |app| {
    app
      .${title}()
      .try_get::<i32>("store", "counter")
      .inspect(|counter| println!("counter: {counter}"))?;
    
    Ok(())
  });

  // It returns an id that can be used to remove the watcher.
  if let Ok(id) = id {
    app.${title}().unwatch("store", id).unwrap();
  }
}
  `;
});
