import { useEffect } from 'react';
import { exit } from '@tauri-apps/plugin-process';
import { onError } from 'example-shared-js/src/index.js';

export function useAppExit() {
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        exit(0).catch(onError);
      }
    };

    window.addEventListener('keydown', onKeyDown);

    return () => {
      window.removeEventListener('keydown', onKeyDown);
    };
  }, []);
}
