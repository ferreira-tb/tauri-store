/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['src/index.ts'],
  out: '../../docs/.vitepress/dist/reference/tauri-plugin-pinia',
  includeVersion: true,
  excludeInternal: true,
  excludePrivate: true,
  excludeProtected: true,
  githubPages: false,
  hideGenerator: true,
  titleLink: 'https://tb.dev.br/tauri-store/pinia/getting-started.html',
};
