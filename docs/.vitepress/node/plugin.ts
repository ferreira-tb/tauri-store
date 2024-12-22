import { cwd } from 'node:process';
import { join, resolve } from 'node:path';
import type { Plugin } from '../../types';
import { parse as parseSemver } from 'semver';
import { pascalCase, snakeCase } from 'change-case';
import { existsSync, readdirSync, readFileSync } from 'node:fs';

interface PackageJson {
  name: string;
  version: string;
}

const PREFIX = 'tauri-plugin-';

export class PluginImpl implements Plugin {
  public readonly name: string;
  public readonly snakeName: string;
  public readonly shortName: string;
  public readonly pascalShortName: string;
  public readonly version: string;
  public readonly shortVersion: string;

  private constructor(json: PackageJson) {
    this.name = json.name;
    this.snakeName = snakeCase(json.name);
    this.shortName = shortName(json.name);
    this.pascalShortName = pascalCase(this.shortName);
    this.version = json.version;
    this.shortVersion = shortVersion(json.version);
  }

  private static create(name: string): Plugin | null {
    const packageJsonPath = join(packagesDir(), name, 'package.json');
    if (existsSync(packageJsonPath)) {
      const packageJson = readFileSync(packageJsonPath, 'utf-8');
      return new PluginImpl(JSON.parse(packageJson));
    }

    return null;
  }

  public static load(): Plugin[] {
    const packages = readdirSync(packagesDir());
    const plugins = packages.filter((name) => name.startsWith(PREFIX));
    return plugins
      .map((name) => PluginImpl.create(name))
      .filter((plugin): plugin is Plugin => Boolean(plugin))
      .toSorted((a, b) => a.name.localeCompare(b.name));
  }
}

function packagesDir() {
  return resolve(cwd(), '../packages');
}

function shortName(name: string) {
  return name.slice(PREFIX.length);
}

function shortVersion(version: string) {
  const parsed = parseSemver(version);
  if (!parsed) {
    throw new Error(`invalid version: ${version}`);
  }

  if (parsed.prerelease.length > 0) {
    return version;
  }

  return `${parsed.major}.${parsed.minor}`;
}
