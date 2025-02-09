import type { Component } from 'svelte';
import compare from 'semver/functions/compare';

const versionFileRegex = /.*?(\d+\.\d+\.\d+.*?)\.svelte$/;

export function sortVersions(versions: Record<string, unknown>): ComponentTuple[] {
  return (Object.entries(versions) as [string, { default: Component }][])
    .map(([version, component]) => [parse(version), component.default] as ComponentTuple)
    .toSorted((a, b) => compare(b[0], a[0]));
}

function parse(name: string) {
  const version = versionFileRegex.exec(name)?.[1];
  if (!version) {
    throw new TypeError(`invalid version file name: ${name}`);
  }

  return version;
}
