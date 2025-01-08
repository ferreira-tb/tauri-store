import type { Component } from 'svelte';
import compare from 'semver/functions/compare';

const versionFileRegex = /.*?(\d+\.\d+\.\d+.*?)\.svelte$/;

export function sortVersions(versions: Record<string, unknown>): GlobComponentTuple[] {
  return (Object.entries(versions) as [string, { default: Component }][])
    .map(([version, component]) => [parse(version), component.default] as GlobComponentTuple)
    .toSorted((a, b) => compare(b[0], a[0]));
}

function parse(name: string) {
  const version = versionFileRegex.exec(name)?.[1];
  if (!version) {
    throw new TypeError(`Invalid version file name: ${name}`);
  }

  return version;
}
