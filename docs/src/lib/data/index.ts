import metadata from './metadata.json';

export const plugins = metadata.filter((it) => it.isPlugin);

export const javascriptDocs = metadata
  .filter((it) => Boolean(it.docs.javascript))
  .map((it) => ({ label: it.name, url: it.docs.javascript! }));

export const rustDocs = metadata
  .filter((it) => Boolean(it.docs.rust))
  .map((it) => ({ label: it.name, url: it.docs.rust }));

export const changelogs = metadata
  .filter((it) => Boolean(it.docs.changelog))
  .map((it) => ({ label: it.name, url: it.docs.changelog }));

export function findMetadata(name: PackageName) {
  return metadata.find((it) => it.name === name)!;
}
