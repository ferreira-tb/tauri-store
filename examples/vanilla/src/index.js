import { exit } from '@tauri-apps/plugin-process';
import { increment, openStore, store } from './store.js';
import { clearAutosave, setAutosave } from 'tauri-store/src/index.js';

export function init() {
  window.addEventListener('DOMContentLoaded', () => {
    const onClick = (id, callback) => {
      const el = document.querySelector(`#${id}`);
      el.addEventListener('click', callback);
    };

    onClick('set-autosave', () => setAutosave(5000));
    onClick('clear-autosave', () => clearAutosave());
    onClick('increment', () => increment());
    onClick('start', () => store.start());
    onClick('stop', () => store.stop());
    onClick('save', () => store.save());
    onClick('save-now', () => store.saveNow());
    onClick('open', () => openStore());

    update(store.state());
    store.subscribe(update);
  });

  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      exit(0);
    }
  });
}

function update(state) {
  const p = document.querySelector('#counter-value');
  p.textContent = `Counter: ${state.counter}`;
}
