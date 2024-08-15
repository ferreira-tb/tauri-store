/** @type {import('typedoc').TypeDocOptions} */
module.exports = {
  entryPoints: ['../guest-js/src/index.ts'],
  out: '../docs/.vitepress/dist/reference',
  includeVersion: true,
  excludePrivate: true,
  excludeProtected: true,
  githubPages: false,
  titleLink: 'https://tb.dev.br/tauri-plugin-pinia/',
};
