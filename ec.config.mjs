import { anchorPlugin } from './src/plugins/anchor.mjs';
import { collapsePlugin } from './src/plugins/collapse.mjs';
import { playgroundPlugin } from './src/plugins/playground.mjs';

/**
 * Expressive Code 配置。插件含函数，不能写进 astro.config.ts。
 * 圆角、边框与背景色对齐全站 custom.css / diagrams.css 的暖炭灰与浅灰体系。
 */
export default {
	plugins: [anchorPlugin, playgroundPlugin, collapsePlugin],
	styleOverrides: {
		borderRadius: '0.625rem',
		borderWidth: '1px',
		borderColor: ({ theme }) => (theme.type === 'dark' ? '#353841' : '#dcdfe6'),
		codeBackground: ({ theme }) => (theme.type === 'dark' ? '#1e2027' : '#f6f7f9'),
		codeFontSize: '0.875rem',
		codeLineHeight: '1.68',
		codePaddingBlock: '0.9rem',
		codePaddingInline: '1.15rem',
		frames: {
			frameBoxShadowCssValue: 'none',
			editorActiveTabBackground: ({ theme }) => (theme.type === 'dark' ? '#1e2027' : '#f6f7f9'),
			editorTabBarBackground: ({ theme }) => (theme.type === 'dark' ? '#17181c' : '#eef0f4'),
			terminalBackground: ({ theme }) => (theme.type === 'dark' ? '#1e2027' : '#f6f7f9'),
			terminalTitlebarBackground: ({ theme }) => (theme.type === 'dark' ? '#17181c' : '#eef0f4'),
		},
	},
};
