import type { Component } from 'svelte';
import compare from 'semver/functions/compare';

const versionFileRegex = /.*?(\d+\.\d+\.\d+.*?)\.svelte$/;

type VersionTuple = [string, Component];

export function sortVersions(versions: Record<string, unknown>): VersionTuple[] {
  return (Object.entries(versions) as [string, { default: Component }][])
    .map(([version, component]) => [fromVersionFile(version), component.default] as VersionTuple)
    .toSorted((a, b) => compare(b[0], a[0]));
}

function fromVersionFile(name: string) {
  const version = versionFileRegex.exec(name)?.[1];
  if (!version) {
    throw new TypeError(`Invalid version file name: ${name}`);
  }

  return version;
}
