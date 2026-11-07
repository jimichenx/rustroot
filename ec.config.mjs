import { collapsePlugin } from './src/plugins/collapse.mjs';
import { playgroundPlugin } from './src/plugins/playground.mjs';

/** Expressive Code 配置。插件含函数，不能写进 astro.config.ts。 */
export default {
	plugins: [playgroundPlugin, collapsePlugin],
};
