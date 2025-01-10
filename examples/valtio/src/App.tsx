/* eslint-disable @typescript-eslint/no-misused-promises */
import { useEffect } from 'react';
import { useSnapshot } from 'valtio';
import { useKeyDown } from 'example-shared-react/src/index.js';
import { onError, printCounter } from 'example-shared-js/src/index.js';
import { clearAutosave, saveAll, saveAllNow, setAutosave } from 'tauri-plugin-valtio/src/index.js';
import {
  debouncedStore,
  incrementCounter,
  incrementDebouncedCounter,
  incrementThrottledCounter,
  openDebouncedStore,
  openStore,
  openThrottledStore,
  store,
  throttledStore,
} from './stores';

export default function App() {
  useKeyDown();
  useEffect(() => {
    // prettier-ignore
    store.start()
      .then(() => debouncedStore.start())
      .then(() => throttledStore.start())
      .catch(onError);
  }, []);

  return (
    <main>
      <div className="action">
        <button type="button" onClick={saveAll}>
          Save All
        </button>
        <button type="button" onClick={saveAllNow}>
          Save All Now
        </button>
        <button type="button" onClick={() => setAutosave(5000)}>
          Set Autosave
        </button>
        <button type="button" onClick={clearAutosave}>
          Clear Autosave
        </button>
      </div>

      <Counter />
      <DebouncedCounter />
      <ThrottledCounter />
    </main>
  );
}

function Counter() {
  const storeState = useSnapshot(store.state);

  return (
    <section id="counter">
      <p>Counter: {storeState.counter}</p>
      <div className="action">
        <button type="button" onClick={incrementCounter}>
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
        <button type="button" onClick={printCounter}>
          Print
        </button>
        <button type="button" onClick={openStore}>
          Open
        </button>
      </div>
    </section>
  );
}

function DebouncedCounter() {
  const debouncedStoreState = useSnapshot(debouncedStore.state);

  return (
    <section id="debounced-counter">
      <p>Debounced Counter: {debouncedStoreState.debouncedCounter}</p>
      <div className="action">
        <button type="button" onClick={incrementDebouncedCounter}>
          Increment
        </button>
        <button type="button" onClick={debouncedStore.start}>
          Start
        </button>
        <button type="button" onClick={debouncedStore.stop}>
          Stop
        </button>
        <button type="button" onClick={debouncedStore.save}>
          Save
        </button>
        <button type="button" onClick={debouncedStore.saveNow}>
          Save Now
        </button>
        <button type="button" onClick={openDebouncedStore}>
          Open
        </button>
      </div>
    </section>
  );
}

function ThrottledCounter() {
  const throttledStoreState = useSnapshot(throttledStore.state);

  return (
    <section id="throttled-counter">
      <p>Throttled Counter: {throttledStoreState.throttledCounter}</p>
      <div className="action">
        <button type="button" onClick={incrementThrottledCounter}>
          Increment
        </button>
        <button type="button" onClick={throttledStore.start}>
          Start
        </button>
        <button type="button" onClick={throttledStore.stop}>
          Stop
        </button>
        <button type="button" onClick={throttledStore.save}>
          Save
        </button>
        <button type="button" onClick={throttledStore.saveNow}>
          Save Now
        </button>
        <button type="button" onClick={openThrottledStore}>
          Open
        </button>
      </div>
    </section>
  );
}
