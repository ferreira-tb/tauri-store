import { commands } from '@tauri-store/shared';

/** @internal */
export const load = commands.load('pinia');

/** @internal */
export const patch = commands.patch('pinia');

/** @internal */
export const unload = commands.unload('pinia');
