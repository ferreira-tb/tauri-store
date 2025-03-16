/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['src/index.ts'],
  out: '../../docs/.vitepress/dist/js-docs/plugin-valtio',
  titleLink: 'https://tb.dev.br/tauri-store/plugin-valtio/guide/getting-started',
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
