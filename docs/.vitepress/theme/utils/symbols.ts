import type { Router } from 'vitepress';
import type { Option } from '@tb-dev/utils';
import type { Plugin } from '../../../types';
import { localRef } from '@/composables/local-ref';
import { data as plugins } from '@/data/plugin.data';
import {
  type App,
  type ComputedRef,
  type InjectionKey,
  inject as original,
  readonly,
  ref,
  type Ref,
  type ShallowRef,
} from 'vue';

export type ComputedSymbol<T> = InjectionKey<ComputedRef<T>>;
export type RefSymbol<T> = InjectionKey<Readonly<Ref<T>>>;
export type ShallowRefSymbol<T> = InjectionKey<Readonly<ShallowRef<T>>>;
export type WritableRefSymbol<T> = InjectionKey<Ref<T>>;
export type WritableShallowRefSymbol<T> = InjectionKey<ShallowRef<T>>;

export const symbols = {
  currentRoute: Symbol('current-route') as RefSymbol<Option<string>>,
  selectedPlugin: Symbol('selected-plugin') as WritableRefSymbol<Plugin>,
} as const;

export function provideSymbols(app: App, router: Router) {
  const defaultPlugin = plugins.find((plugin) => plugin.name.includes('pinia'));
  const selectedPlugin = localRef('tauri:selected-plugin', defaultPlugin ?? plugins[0]);
  app.provide(symbols.selectedPlugin, selectedPlugin);

  const currentRoute = ref<Option<string>>();
  router.onAfterRouteChanged = (to) => {
    currentRoute.value = to;
  };

  app.provide(symbols.currentRoute, readonly(currentRoute));
}

export function inject<T>(key: InjectionKey<T>): T {
  const value = original(key);
  if (typeof value === 'undefined') {
    throw new TypeError('injection failed: value was not provided');
  }

  return value;
}
