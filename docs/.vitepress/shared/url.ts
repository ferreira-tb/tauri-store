import type { Plugin } from '../../types';

export function docsRs(plugin: Plugin | string, extra?: string) {
  const name = typeof plugin === 'string' ? plugin : plugin.snakeName;
  const url = `https://tb.dev.br/tauri-store/rust-docs/${name}`;
  return extra ? `${url}/${extra}` : url;
}

export function reference(plugin: Plugin, extra?: string) {
  const url = `https://tb.dev.br/tauri-store/reference/${plugin.name}`;
  return extra ? `${url}/${extra}` : `${url}/index.html`;
}
