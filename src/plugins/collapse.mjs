import { codeLineClass } from '@expressive-code/core';
import { addClassName, h, selectAll } from '@expressive-code/core/hast';

const en = (process.env.SITE_LOCALE ?? 'zh').toLowerCase() === 'en';

/**
 * `<Code collapse={行数}>` 和围栏 `collapse=行数`：行数超过这个值时，
 * 多出来的行先折起来，块底有按钮可以展开、再收起。
 * 不超过这个行数时不显示按钮。复制和 Playground 仍是完整源码。
 * 必须放在 ec.config.mjs：插件含函数，写进 astro.config.ts 后 `<Code>` 无法序列化配置。
 */
export const collapsePlugin = {
	name: 'Collapse long code',
	baseStyles: ({ cssVar }) => `
		.frame.has-fold > pre {
			border-end-start-radius: 0;
			border-end-end-radius: 0;
		}

		.frame.has-fold:not(.is-open) .${codeLineClass}.is-folded {
			display: none;
		}

		.fold {
			appearance: none;
			position: relative;
			z-index: 1;
			display: flex;
			align-items: center;
			justify-content: center;
			box-sizing: border-box;
			width: 100%;
			margin: 0;
			margin-block-start: calc(${cssVar('borderWidth')} * -1);
			padding: 0.45rem 0.75rem;
			border: ${cssVar('borderWidth')} solid ${cssVar('borderColor')};
			border-end-start-radius: calc(${cssVar('borderRadius')} + ${cssVar('borderWidth')});
			border-end-end-radius: calc(${cssVar('borderRadius')} + ${cssVar('borderWidth')});
			background: var(--code-background);
			color: ${cssVar('gutterForeground')};
			font-family: ${cssVar('uiFontFamily')};
			font-size: ${cssVar('uiFontSize')};
			font-weight: ${cssVar('uiFontWeight')};
			line-height: 1.4;
			cursor: pointer;
		}

		.frame.has-fold:not(.is-open) > .fold::before {
			content: '';
			position: absolute;
			inset-inline: 0;
			bottom: 100%;
			height: 1.75rem;
			background: linear-gradient(to bottom, transparent, var(--code-background));
			pointer-events: none;
		}

		.fold:hover,
		.fold:focus-visible {
			color: ${cssVar('codeForeground')};
			background: color-mix(in srgb, ${cssVar('codeForeground')} 8%, var(--code-background));
		}

		.fold:focus-visible {
			outline: 2px solid ${cssVar('focusBorder')};
			outline-offset: -3px;
		}

		@media print {
			.frame.has-fold .${codeLineClass}.is-folded {
				display: grid;
			}

			.frame.has-fold > .fold {
				display: none;
			}
		}
	`,
	jsModules: [
		`
		if (!globalThis.__rrCodeFold) {
			globalThis.__rrCodeFold = true;
			document.addEventListener('click', (event) => {
				const target = event.target;
				if (!(target instanceof Element)) return;
				const button = target.closest('.expressive-code .fold');
				if (!(button instanceof HTMLButtonElement)) return;
				const frame = button.closest('.frame');
				if (!frame) return;
				const open = frame.classList.toggle('is-open');
				button.setAttribute('aria-expanded', open ? 'true' : 'false');
				const label = button.querySelector('.fold-label');
				const next = button.getAttribute(open ? 'data-label-open' : 'data-label-closed');
				if (label && next) label.textContent = next;
			});
		}
		`,
	],
	hooks: {
		preprocessMetadata({ codeBlock }) {
			const specified = codeBlock.metaOptions.list('collapse');
			if (!specified.length) return;
			const fromMeta = codeBlock.metaOptions.getInteger('collapse');
			if (fromMeta === undefined || fromMeta < 1) {
				throw new Error(
					`代码块的 collapse 必须是正整数，收到的是 ${specified.at(-1)?.raw ?? ''}。`,
				);
			}
			codeBlock.props.collapse = fromMeta;
		},
		postprocessRenderedBlock({ codeBlock, renderData }) {
			const visible = parseCollapse(codeBlock.props.collapse);
			if (visible === undefined) return;

			const root = renderData.blockAst;
			if (root.tagName !== 'figure') return;

			const lines = selectAll(`.${codeLineClass}`, root);
			const hidden = lines.length - visible;
			if (hidden < 1) return;

			for (const line of lines.slice(visible)) addClassName(line, 'is-folded');
			addClassName(root, 'has-fold');

			const closedLabel = en
				? hidden === 1
					? 'Show 1 more line'
					: `Show ${hidden} more lines`
				: `展开其余 ${hidden} 行`;
			const openLabel = en ? 'Show less' : '收起';

			root.children.push(
				h(
					'button',
					{
						type: 'button',
						className: ['fold'],
						ariaExpanded: 'false',
						dataLabelClosed: closedLabel,
						dataLabelOpen: openLabel,
					},
					[h('span', { className: ['fold-label'] }, closedLabel)],
				),
			);
		},
	},
};

function parseCollapse(value) {
	if (value === undefined || value === null || value === '') return undefined;
	const n = typeof value === 'number' ? value : typeof value === 'string' ? Number(value.trim()) : NaN;
	if (!Number.isInteger(n) || n < 1) {
		throw new Error(`代码块的 collapse 必须是正整数，收到的是 ${JSON.stringify(value)}。`);
	}
	return n;
}
