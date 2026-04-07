/** @type {import('typedoc').TypeDocOptions} */
export default {
  entryPoints: ['src/index.ts'],
  out: '../../docs/.vitepress/dist/js-docs/plugin-pinia',
  titleLink: 'https://tb.dev.br/tauri-store/plugin-pinia/guide/getting-started',
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
