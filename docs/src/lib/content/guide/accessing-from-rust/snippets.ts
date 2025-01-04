import { snakeCase } from 'change-case';
import { snippet } from '$lib/stores/snippet';

export const get = snippet((metadata) => {
  return `
use ${snakeCase(metadata.name)}::ManagerExt;

#[tauri::command]
async fn get_counter(app: AppHandle) -> i32 {
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
#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .${snakeCase(metadata.title!)}()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
  `;
});
