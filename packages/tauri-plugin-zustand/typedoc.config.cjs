/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['src/index.ts'],
  out: '../../docs/dist/reference/tauri-plugin-zustand',
  titleLink: 'https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-plugin-zustand',
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
