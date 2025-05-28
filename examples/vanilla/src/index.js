import { printStore } from './commands.js';
import { exit } from '@tauri-apps/plugin-process';
import { clearAutosave, setAutosave } from 'tauri-store/src/index.js';
import { increment, incrementNested, openStore, store } from './store.js';

export function init() {
  window.addEventListener('DOMContentLoaded', () => {
    const onClick = (id, callback) => {
      const el = document.querySelector(`#${id}`);
      el.addEventListener('click', callback);
    };

    onClick('set-autosave', () => setAutosave(5000));
    onClick('clear-autosave', () => clearAutosave());
    onClick('increment', () => increment());
    onClick('increment-nested', () => incrementNested());
    onClick('start', () => store.start());
    onClick('stop', () => store.stop());
    onClick('save', () => store.save());
    onClick('save-now', () => store.saveNow());
    onClick('print', () => printStore());
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
  const counter = state.counter;
  const nested = state.nested.foo.bar.baz;
  const p = document.querySelector('#counter-value');
  p.textContent = `Counter: ${counter} Nested: ${nested}`;
}
