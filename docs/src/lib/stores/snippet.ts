import type { Code } from '$lib/code';
import { derived } from 'svelte/store';
import { currentMetadata, type Metadata } from '$lib/stores/plugin';

type SnippetFn = (metadata: Metadata) => string;

export function snippet(fn: SnippetFn) {
  return derived(currentMetadata, derive(fn), null);
}

function derive(fn: SnippetFn) {
  return function (current: Metadata | null) {
    return current ? fn(current) : null;
  };
}

type SnippetGroupFn = (metadata: Metadata) => Code | Code[];

export function snippetGroup(fn: SnippetGroupFn) {
  return derived(currentMetadata, deriveGroup(fn), []);
}

function deriveGroup(fn: SnippetGroupFn) {
  return (current: Metadata) => fn(current);
}
