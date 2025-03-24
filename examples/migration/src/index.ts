import { type State, store } from './store';
import { exit } from '@tauri-apps/plugin-process';

export function init() {
  window.addEventListener('DOMContentLoaded', () => {
    onLoad().catch(console.error);
  });

  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      exit(0).catch(console.error);
    }
  });
}

async function onLoad() {
  await store.start();
  update(store.state());
  store.subscribe(update);
}

function update(state: State) {
  const pre = document.querySelector('#content')!;
  pre.textContent = JSON.stringify(state, null, 2);
}
