/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['src/lib/index.ts'],
  out: '../../docs/.vitepress/dist/js-docs/plugin-svelte',
  titleLink: 'https://tb.dev.br/tauri-store/plugin-svelte/guide/getting-started',
  includeVersion: true,
  excludeInternal: true,
  excludePrivate: true,
  excludeProtected: true,
  githubPages: false,
  hideGenerator: true,
  markdownLinkExternal: true,
  sourceLinkExternal: true,
  treatWarningsAsErrors: true,
};
