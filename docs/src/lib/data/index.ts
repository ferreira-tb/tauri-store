import metadata from './metadata.json';

export const plugins = metadata.filter((it) => it.isPlugin);

interface MetadataUrl {
  label: string;
  url: string;
}

export const javascriptDocs: MetadataUrl[] = metadata
  .filter((it) => Boolean(it.docs.javascript))
  .map((it) => ({ label: it.name, url: it.docs.javascript! }));

export const rustDocs: MetadataUrl[] = metadata
  .filter((it) => Boolean(it.docs.rust))
  .map((it) => ({ label: it.name, url: it.docs.rust }));

export const changelogs: MetadataUrl[] = metadata
  .filter((it) => Boolean(it.docs.changelog))
  .map((it) => ({ label: it.name, url: it.docs.changelog }));

javascriptDocs.sort(sortMetadataUrl);
rustDocs.sort(sortMetadataUrl);
changelogs.sort(sortMetadataUrl);

function sortMetadataUrl(a: MetadataUrl, b: MetadataUrl) {
  if (a.label === 'tauri-store') return -1;
  if (b.label === 'tauri-store') return 1;
  return a.label.localeCompare(b.label);
}

export function findMetadata(name: PackageName) {
  return metadata.find((it) => it.name === name)!;
}
