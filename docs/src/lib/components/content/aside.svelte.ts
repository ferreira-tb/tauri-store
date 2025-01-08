import { page } from '$app/state';
import { onMount, untrack } from 'svelte';

interface Heading {
  element: Element;
  level: number;
  link: string;
  title: string;
}

class Headings {
  public value: Heading[] = $state.raw([]);

  public get() {
    this.value = Array.from(document.querySelectorAll('main h2'))
      .filter((el) => el.id && el.hasChildNodes() && el.textContent)
      .map((el) => {
        return {
          element: el,
          title: el.textContent!.trim(),
          link: `#${el.id}`,
          level: Number.parseInt(el.tagName[1]),
        };
      });
  }

  public update() {
    const get = this.get.bind(this);
    if (typeof requestIdleCallback === 'function') {
      requestIdleCallback(get, { timeout: 100 });
    } else {
      setTimeout(get, 0);
    }
  }

  public cleanup() {
    this.value = [];
  }
}

const headings = new Headings();

export function useHeadings() {
  $effect(() => {
    const _ = page.url;
    untrack(() => headings.update());
  });

  onMount(() => {
    headings.update();
    return headings.cleanup.bind(headings);
  });

  return headings;
}

export type { Headings };
