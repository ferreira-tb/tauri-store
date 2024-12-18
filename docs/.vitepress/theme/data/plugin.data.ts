import { defineLoader } from 'vitepress';
import type { Plugin } from '../../../types';
import { PluginImpl } from '../../node/plugin';

declare const data: Plugin[];
export { data, type Plugin };

export default defineLoader({
  load: PluginImpl.load.bind(PluginImpl),
});
