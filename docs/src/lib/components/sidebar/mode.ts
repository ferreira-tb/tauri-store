import { persistent } from '$lib/stores/persistent';

export type Mode = 'learn' | 'reference' | 'changelog';

const STORAGE_KEY = 'sidebar-mode';
export const DEFAULT_MODE: Mode = 'learn';

class SidebarMode {
  private readonly current = persistent<Mode>(STORAGE_KEY, DEFAULT_MODE);
  public readonly subscribe = this.current.subscribe.bind(this.current);
  public readonly set = this.current.set.bind(this.current);
}

export const mode = new SidebarMode();
