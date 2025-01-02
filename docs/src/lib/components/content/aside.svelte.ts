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
  public lastRoute: string | null = null;

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
}

const headings = new Headings();

export function useHeadings() {
  $effect(() => {
    const url = page.url.toString();
    if (headings.lastRoute !== url) {
      untrack(() => update());
    }

    return cleanup;
  });

  onMount(() => {
    update();
    return cleanup;
  });

  return headings;
}

function cleanup() {
  headings.value = [];
}

function update() {
  const get = headings.get.bind(headings);
  if (typeof requestIdleCallback === 'function') {
    requestIdleCallback(get, { timeout: 100 });
  } else {
    setTimeout(get, 0);
  }
}

export type { Headings };
