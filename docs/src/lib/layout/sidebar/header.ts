import type { Mode } from './mode';
import { BookOpen, Code, FileClock } from 'lucide-svelte';

export function resolveIcon(mode: Mode | null) {
  switch (mode) {
    case 'changelog': {
      return FileClock;
    }
    case 'reference': {
      return Code;
    }
    case 'learn':
    default: {
      return BookOpen;
    }
  }
}
