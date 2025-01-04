import { type Subscriber, writable } from 'svelte/store';

class Persistent<T extends string = string> {
  private readonly key: string;
  private readonly value = writable<T | null>(null, this.load.bind(this));
  private readonly defaultValue: T | null;

  constructor(key: string, defaultValue: T | null = null) {
    this.key = `tauri-store:${key}`;
    this.defaultValue = defaultValue;
  }

  public subscribe(run: Subscriber<T | null>, invalidate?: () => void) {
    return this.value.subscribe(run, invalidate);
  }

  public set(value: T) {
    this.value.set(value);
    if (value.length === 0) {
      localStorage.removeItem(this.key);
    } else {
      localStorage.setItem(this.key, value);
    }
  }

  private load() {
    let value = localStorage.getItem(this.key) as T | null;
    if (value?.length === 0) value = null;
    if (!value && this.defaultValue) value = this.defaultValue;
    this.value.set(value);
  }
}

export function persistent<T extends string = string>(key: string, defaultValue?: T) {
  return new Persistent<T>(key, defaultValue);
}
