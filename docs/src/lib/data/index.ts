import metadata from './metadata.json';

export const plugins = metadata;

type MetadataUrl = {
  label: string;
  url: string;
};

export const javascriptDocs: MetadataUrl[] = plugins.map((it) => {
  return { label: it.name, url: it.docs.javascript };
});

export const rustDocs: MetadataUrl[] = plugins.map((it) => {
  return { label: it.name, url: it.docs.rust };
});

export const changelogs: MetadataUrl[] = plugins.map((it) => {
  return { label: it.name, url: it.docs.changelog };
});

export function findMetadata(name: TauriPlugin) {
  return plugins.find((it) => it.name === name)!;
}

const sortPlugin = sort((it: (typeof plugins)[0]) => it.name);
plugins.sort(sortPlugin);

const sortMetadataUrl = sort((it: MetadataUrl) => it.label);
javascriptDocs.sort(sortMetadataUrl);
rustDocs.sort(sortMetadataUrl);
changelogs.sort(sortMetadataUrl);

function sort<T>(fn: (value: T) => string) {
  return (a: T, b: T) => {
    if (fn(a) === 'tauri-store') return -1;
    if (fn(b) === 'tauri-store') return 1;
    return fn(a).localeCompare(fn(b));
  };
}
