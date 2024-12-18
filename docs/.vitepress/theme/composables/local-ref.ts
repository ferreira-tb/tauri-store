import { computed, type Ref } from 'vue';
import { useLocalStorage } from '@vueuse/core';

interface Value<T> {
  inner: T;
}

export function localRef<T>(key: string, initialValue: T): Ref<T> {
  const defaultValue: Value<T> = { inner: initialValue };
  const local = useLocalStorage<Value<T>>(key, defaultValue, {
    writeDefaults: true,
    shallow: false,
    deep: true,
    serializer: {
      read: JSON.parse,
      write: JSON.stringify,
    },
  });

  return computed<T>({
    get: () => local.value.inner,
    set: (value: T) => {
      local.value.inner = value;
    },
  });
}
