import { kebabCase } from 'change-case';
import type { Component } from 'svelte';

const filenameRegex = /^\.\/(.+?)\.svelte$/;

export function findByTitle(files: Record<string, unknown>, title: string | null) {
  if (!title) return null;
  title = kebabCase(title);
  for (const [file, component] of Object.entries(files)) {
    const filename = filenameRegex.exec(file)?.[1];
    if (filename === title) {
      return (component as { default: Component }).default;
    }
  }

  return null;
}
