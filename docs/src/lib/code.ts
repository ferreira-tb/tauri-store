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

class Highlighter {
  private core: HighlighterCore | null = null;
  private loading: Promise<HighlighterCore> | null = null;

  public async load(): Promise<HighlighterCore> {
    if (this.core) return this.core;
    if (this.loading) return this.loading;

    this.loading = createHighlighterCore({
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

    return this.loading.then((core) => {
      this.core = core;
      this.loading = null;
      return core;
    });
  }
}

const highlighter = new Highlighter();

export async function highlight(code: string, options: HighlighterOptions) {
  const core = await highlighter.load();
  return core.codeToHtml(code, {
    ...options,
    theme: get(mode) === 'light' ? 'vitesse-light' : 'vitesse-dark',
  });
}
