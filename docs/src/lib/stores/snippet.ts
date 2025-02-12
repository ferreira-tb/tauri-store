import type { Code } from '$lib/code';
import { derived } from 'svelte/store';
import { snakeCase } from 'change-case';
import { currentMetadata, type Metadata } from '$stores/plugin';

type SnippetFn = (metadata: Metadata, ctx: Context) => string;

interface Context {
  collection: string;
  isTauriStore: boolean;
}

export function snippet(fn: SnippetFn) {
  return derived(currentMetadata, derive(fn), null);
}

function derive(fn: SnippetFn) {
  return function (current: Metadata | null) {
    return current ? fn(current, context(current)) : null;
  };
}

type SnippetGroupFn = (metadata: Metadata, ctx: Context) => Code | Code[];

export function snippetGroup(fn: SnippetGroupFn) {
  return derived(currentMetadata, deriveGroup(fn), []);
}

function deriveGroup(fn: SnippetGroupFn) {
  return (current: Metadata) => fn(current, context(current));
}

function context(metadata: Metadata): Context {
  const name = metadata.name as TauriPlugin;
  const isTauriStore = name === 'tauri-store';
  return {
    isTauriStore,
    collection: isTauriStore ? 'store_collection' : snakeCase(metadata.title),
  };
}
