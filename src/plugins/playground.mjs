import { codeLineClass, createInlineSvgUrl } from '@expressive-code/core';
import { addClassName, h } from '@expressive-code/core/hast';

const RUST_LANGS = new Set(['rust', 'rs']);

const label =
	(process.env.SITE_LOCALE ?? 'zh').toLowerCase() === 'en'
		? 'Run in the Rust Playground'
		: '在 Rust Playground 中运行';

const playIcon = createInlineSvgUrl(
	`<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path fill='black' d='M8.2 5.6v12.8L18.6 12 8.2 5.6z'/></svg>`,
);

/**
 * 给 rust 代码块加一个链接，在官方 Playground 打开同一段源码。
 * 课节里的 `<Code lang="rust">` 和 ```rust 围栏都会经过这里。
 * 必须放在 ec.config.mjs：插件含函数，写进 astro.config.ts 后 `<Code>` 无法序列化配置。
 */
export const playgroundPlugin = {
	name: 'Rust Playground',
	baseStyles: ({ cssVar }) => `
		.playground {
			position: absolute;
			z-index: 1;
			inset-block-start: calc(${cssVar('borderWidth')} + var(--button-spacing));
			inset-inline-end: calc(${cssVar('borderWidth')} + ${cssVar('uiPaddingInline')} / 2 + 2.75rem);
			width: 2.5rem;
			height: 2.5rem;
			margin: 0;
			border-radius: 0.2rem;
			background: var(--code-background);
			opacity: 0.75;
			direction: ltr;
			unicode-bidi: isolate;
		}

		.playground::before {
			content: '';
			position: absolute;
			pointer-events: none;
			inset: 0;
			border-radius: inherit;
			border: ${cssVar('borderWidth')} solid ${cssVar('frames.inlineButtonBorder')};
			opacity: ${cssVar('frames.inlineButtonBorderOpacity')};
		}

		.playground::after {
			content: '';
			position: absolute;
			pointer-events: none;
			inset: 0;
			margin: 0.55rem;
			background-color: ${cssVar('frames.inlineButtonForeground')};
			-webkit-mask-image: ${playIcon};
			mask-image: ${playIcon};
			-webkit-mask-repeat: no-repeat;
			mask-repeat: no-repeat;
			-webkit-mask-position: center;
			mask-position: center;
			-webkit-mask-size: contain;
			mask-size: contain;
		}

		.playground:hover,
		.playground:focus-visible {
			opacity: 1;
		}

		.playground:hover::before,
		.playground:focus-visible::before {
			background: ${cssVar('frames.inlineButtonBackground')};
			opacity: ${cssVar('frames.inlineButtonBackgroundHoverOrFocusOpacity')};
		}

		.frame.has-playground :nth-child(1 of .${codeLineClass}) .code {
			padding-inline-end: calc(5.5rem + ${cssVar('codePaddingInline')});
		}

		@media (hover: hover) {
			.playground {
				opacity: 0;
				width: 2rem;
				height: 2rem;
				inset-inline-end: calc(${cssVar('borderWidth')} + ${cssVar('uiPaddingInline')} / 2 + 2.25rem);
			}

			.frame:hover .playground,
			.frame:focus-within .playground {
				opacity: 0.75;
			}

			.frame.has-playground :nth-child(1 of .${codeLineClass}) .code {
				padding-inline-end: calc(4.5rem + ${cssVar('codePaddingInline')});
			}
		}
	`,
	hooks: {
		postprocessRenderedBlock({ codeBlock, renderData }) {
			const language = codeBlock.language?.toLowerCase();
			if (!language || !RUST_LANGS.has(language)) return;
			if (!codeBlock.code.trim()) return;

			const href =
				'https://play.rust-lang.org/?' +
				new URLSearchParams({
					version: 'stable',
					mode: 'debug',
					edition: '2024',
					code: codeBlock.code,
				}).toString();

			const root = renderData.blockAst;
			if (root.tagName === 'figure') addClassName(root, 'has-playground');
			root.children.push(
				h(
					'a',
					{
						className: ['playground'],
						href,
						target: '_blank',
						rel: 'noopener noreferrer',
						title: label,
					},
					[h('span', { className: ['sr-only'] }, label)],
				),
			);
		},
	},
};
