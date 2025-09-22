/* eslint-disable @typescript-eslint/no-floating-promises */
/* eslint-disable @typescript-eslint/no-misused-promises */
import { useEffect } from 'react';
import { exit } from '@tauri-apps/plugin-process';
import { openStore, tauriHandler, useCounterStore } from './store';
import { clearAutosave, setAutosave } from '@tauri-store/zustand/src/index.js';

export default function App() {
  const counter = useCounterStore((state) => state.counter);
  const increment = useCounterStore((state) => state.increment);

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
        <p>Counter: {counter}</p>
        <div className="action">
          <button type="button" onClick={increment}>
            Increment
          </button>
          <button type="button" onClick={() => tauriHandler.start()}>
            Start
          </button>
          <button type="button" onClick={() => tauriHandler.stop()}>
            Stop
          </button>
          <button type="button" onClick={() => tauriHandler.save()}>
            Save
          </button>
          <button type="button" onClick={() => tauriHandler.saveNow()}>
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
