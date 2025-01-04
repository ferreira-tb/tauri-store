import { tick } from 'svelte';
import { get } from 'svelte/store';
import { mode } from 'mode-watcher';
import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import { createHighlighterCore, type HighlighterCore } from 'shiki/core';

export type Lang =
  | 'json'
  | 'powershell'
  | 'rust'
  | 'shell'
  | 'svelte'
  | 'toml'
  | 'tsx'
  | 'typescript'
  | 'vue';

export interface Code {
  id?: string;
  label: string;
  lang: Lang;
  value: string | null;
}

interface HighlighterOptions {
  lang: Lang;
}

function highlighterFactory() {
  let core: HighlighterCore | null = null;
  let loadPromise: Promise<unknown> | null = null;

  return async (): Promise<HighlighterCore> => {
    await loadPromise;
    loadPromise &&= null;

    if (!core) {
      loadPromise = new Promise((resolve, reject) => {
        create()
          .then((it) => void (core = it))
          .then(resolve, reject);
      });

      await loadPromise;
    }

    await tick();

    if (core) return core;
    throw new Error('failed to load highlighter');
  };
}

function create() {
  return createHighlighterCore({
    engine: createOnigurumaEngine(import('shiki/wasm')),
    themes: [import('shiki/themes/vitesse-dark.mjs'), import('shiki/themes/vitesse-light.mjs')],
    langs: [
      import('shiki/langs/json.mjs'),
      import('shiki/langs/powershell.mjs'),
      import('shiki/langs/rust.mjs'),
      import('shiki/langs/shell.mjs'),
      import('shiki/langs/svelte.mjs'),
      import('shiki/langs/toml.mjs'),
      import('shiki/langs/tsx.mjs'),
      import('shiki/langs/typescript.mjs'),
      import('shiki/langs/vue.mjs'),
    ],
  });
}

const highlighter = highlighterFactory();

export async function highlight(code: string, options: HighlighterOptions) {
  const core = await highlighter();
  return core.codeToHtml(code, {
    ...options,
    theme: get(mode) === 'light' ? 'vitesse-light' : 'vitesse-dark',
  });
}
