import type { Element, Parents, Text } from 'hast';
import { defineHastPlugin, type MdxJsxFlowElementHast, type MdxJsxTextElementHast } from 'satteri';
import { REPOSITORY } from '../config/site';

const BLOB = `${REPOSITORY}/blob/develop/`;

type JsxElement = MdxJsxFlowElementHast | MdxJsxTextElementHast;

/**
 * 代码块上方的「源文件」小字仍写公开库根目录下的完整路径。
 * 页面只显示文件名，并链到 develop 分支上的该文件。
 * MDX 里的 `<code>` 是 JSX 节点，不是 Markdown 行内代码。
 */
export const exampleSourcePlugin = defineHastPlugin({
	name: 'rustroot-example-source',
	element: {
		filter: ['code'],
		visit(node, ctx) {
			linkExample(node, ctx.parent(node), ctx);
		},
	},
	mdxJsxTextElement: {
		filter: ['code'],
		visit(node, ctx) {
			linkExample(node, ctx.parent(node), ctx);
		},
	},
	mdxJsxFlowElement: {
		filter: ['code'],
		visit(node, ctx) {
			linkExample(node, ctx.parent(node), ctx);
		},
	},
});

function linkExample(
	node: Element | JsxElement,
	parent: Parents | undefined,
	ctx: { textContent(node: Element | JsxElement): string; replaceNode(node: Element | JsxElement, next: Element): void },
) {
	if (!parent || !isCodeSource(parent)) return;

	const path = ctx.textContent(node).trim();
	if (!isExamplePath(path)) return;

	const name = path.slice(path.lastIndexOf('/') + 1);
	ctx.replaceNode(node, {
		type: 'element',
		tagName: 'a',
		properties: {
			href: BLOB + path,
			target: '_blank',
			rel: ['noopener', 'noreferrer'],
		},
		children: [
			{
				type: 'element',
				tagName: 'code',
				properties: {},
				children: [text(name)],
			},
		],
	});
}

function isCodeSource(node: Parents) {
	if (node.type === 'element') return node.tagName === 'small' && hasClass(node, 'code-source');
	if (node.type !== 'mdxJsxFlowElement' && node.type !== 'mdxJsxTextElement') return false;
	return node.name === 'small' && jsxClass(node).includes('code-source');
}

function jsxClass(node: JsxElement) {
	const values: string[] = [];
	for (const attribute of node.attributes) {
		if (attribute.type !== 'mdxJsxAttribute') continue;
		if (attribute.name !== 'class' && attribute.name !== 'className') continue;
		if (typeof attribute.value === 'string') values.push(attribute.value);
	}
	return values.join(' ').split(/\s+/);
}

function isExamplePath(path: string) {
	return path.startsWith('examples/') && !path.includes('..') && !path.endsWith('/');
}

function text(value: string): Text {
	return { type: 'text', value };
}

function hasClass(node: Element, name: string) {
	const className = node.properties?.className;
	if (Array.isArray(className)) return className.includes(name);
	return className === name;
}
