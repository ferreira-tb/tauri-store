import type { CloneFn } from './types';
import type { Fn, State } from '@tauri-store/shared';

enum EventKind {
  Change = 'change',
  ChangeKey = 'changekey',
}

export class EventEmitter<S extends State> extends EventTarget {
  private readonly map: Map<keyof S, unknown>;
  private readonly options: ObservableOptions;

  private readonly listeners = {
    [EventKind.Change]: 0,
    [EventKind.ChangeKey]: 0,
  };

  constructor(state: S, options: ObservableOptions) {
    super();

    this.options = options;
    this.map = new Map(
      Array.from(Object.entries(state)).map(([key, value]) => {
        return [key, this.options.clone(value)];
      })
    );
  }

  private emit(e: EventKind.Change): void;
  private emit<K extends keyof S>(e: EventKind.ChangeKey, detail: KeyEvent<S, K>): void;
  private emit(e: EventKind, detail?: unknown): void {
    if (this.listeners[e] > 0) {
      switch (e) {
        case EventKind.Change:
          this.dispatch(e, this.state());
          break;
        case EventKind.ChangeKey:
          this.dispatch(e, detail);
          break;
      }
    }
  }

  private dispatch(e: EventKind, detail?: unknown): void {
    schedule(() => void this.dispatchEvent(new CustomEvent(e, { detail })));
  }

  public subscribe(fn: (state: S) => void): Fn {
    const listener = (event: CustomEvent<S>) => {
      fn(this.clone(event.detail));
    };

    this.addEventListener(EventKind.Change, listener);
    this.listeners[EventKind.Change]++;

    return () => {
      this.removeEventListener(EventKind.Change, listener);
      this.listeners[EventKind.Change]--;
    };
  }

  public subscribeKey<K extends keyof S>(key: K, fn: (value: S[K]) => void): Fn {
    const listener = (event: CustomEvent<KeyEvent<S, K>>) => {
      if (event.detail.key === key) {
        fn(this.clone(event.detail.value));
      }
    };

    this.addEventListener(EventKind.ChangeKey, listener);
    this.listeners[EventKind.ChangeKey]++;

    return () => {
      this.removeEventListener(EventKind.ChangeKey, listener);
      this.listeners[EventKind.ChangeKey]--;
    };
  }

  public state(): S {
    const state = Object.create(null);
    this.map.forEach((value, key) => {
      Reflect.set(state, key, this.clone(value));
    });

    return state as S;
  }

  public get<K extends keyof S>(key: K): S[K] {
    return this.clone(this.map.get(key)) as S[K];
  }

  public set<K extends keyof S>(key: K, value: S[K], emit: boolean): void {
    this.map.set(key, this.clone(value));
    if (emit) {
      this.emit(EventKind.ChangeKey, { key, value });
      this.emit(EventKind.Change);
    }
  }

  public update<K extends keyof S>(key: K, fn: (value: S[K]) => S[K], emit: boolean): void {
    const value = fn(this.map.get(key) as S[K]);
    this.set(key, value, emit);
  }

  public patch(state: Partial<S>, emit: boolean): void {
    let changed = false;
    for (const [key, value] of Object.entries(state)) {
      this.map.set(key as keyof S, this.clone(value));
      changed ||= true;

      if (emit) {
        this.emit(EventKind.ChangeKey, { key, value });
      }
    }

    if (changed && emit) {
      this.emit(EventKind.Change);
    }
  }

  public size(): number {
    return this.map.size;
  }

  public has(key: string): boolean {
    return this.map.has(key);
  }

  public keys(): (keyof S)[] {
    return Array.from(this.map.keys());
  }

  public values(): S[keyof S][] {
    const clone = (value: unknown) => this.clone(value);
    return Array.from(this.map.values()).map(clone) as S[keyof S][];
  }

  private get clone(): <T>(value: T) => T {
    return this.options.clone;
  }
}

function schedule(fn: Fn): void {
  setTimeout(fn, 0);
}

type ObservableOptions = {
  clone: CloneFn;
};

type KeyEvent<S extends State, K extends keyof S> = {
  key: K;
  value: S[K];
};
