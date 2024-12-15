import { inject, symbols } from '../utils/symbols';

export function useSelectedPlugin() {
  return inject(symbols.selectedPlugin);
}
