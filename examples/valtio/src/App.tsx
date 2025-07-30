/* eslint-disable @typescript-eslint/no-floating-promises */
/* eslint-disable @typescript-eslint/no-misused-promises */
import { useEffect } from 'react';
import { useSnapshot } from 'valtio';
import { exit } from '@tauri-apps/plugin-process';
import { increment, openStore, store } from './store';
import { clearAutosave, setAutosave } from '@tauri-store/valtio/src/index.js';

export default function App() {
  const storeState = useSnapshot(store.state);

  useEffect(() => {
    window.addEventListener('keydown', onKeyDown);
    return () => {
      window.removeEventListener('keydown', onKeyDown);
    };
  }, []);

  return (
    <main>
      <div className="action">
        <button type="button" onClick={() => setAutosave(5000)}>
          Set Autosave
        </button>
        <button type="button" onClick={clearAutosave}>
          Clear Autosave
        </button>
      </div>

      <section id="counter">
        <p>Counter: {storeState.counter}</p>
        <div className="action">
          <button type="button" onClick={increment}>
            Increment
          </button>
          <button type="button" onClick={store.start}>
            Start
          </button>
          <button type="button" onClick={store.stop}>
            Stop
          </button>
          <button type="button" onClick={store.save}>
            Save
          </button>
          <button type="button" onClick={store.saveNow}>
            Save Now
          </button>
          <button type="button" onClick={openStore}>
            Open
          </button>
        </div>
      </section>
    </main>
  );
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault();
    exit(0);
  }
}
