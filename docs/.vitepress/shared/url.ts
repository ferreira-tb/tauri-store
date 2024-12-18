import type { Plugin } from '../../types';

export function docsRs(plugin: Plugin, extra?: string) {
  const url = `https://docs.rs/${plugin.name}/latest/${plugin.snakeName}`;
  return extra ? `${url}/${extra}` : url;
}

export function reference(plugin: Plugin, extra?: string) {
  const url = `https://tb.dev.br/tauri-store/reference/${plugin.name}`;
  return extra ? `${url}/${extra}` : `${url}/index.html`;
}
