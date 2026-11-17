import type { Element, Parents, Text } from 'hast';
import { defineHastPlugin, type MdxJsxFlowElementHast, type MdxJsxTextElementHast } from 'satteri';
import { REPOSITORY } from '../config/site';

const BLOB = `${REPOSITORY}/blob/main/`;

type JsxElement = MdxJsxFlowElementHast | MdxJsxTextElementHast;

/**
 * 代码块上方的「源文件」小字仍写公开库根目录下的完整路径。
 * 页面只显示文件名，并链到 main 分支上的该文件。
 * 整行只有「源文件：」或 `Source:` 加文件名时，链接带 `code-source-file`，样式把整行靠右。
 * 后面还有「（节选）」或别的说明时不加，那些是代码块标题，继续贴左缘。
 * 小字节点是冻结的，类加在替换出来的链接上。
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
			...(isFileCredit(parent) ? { className: ['code-source-file'] } : {}),
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

/**
 * 整行只有一处代码路径，其余文字是「源文件：」或 `Source:`。
 * 访问到的 code 节点和 parent.children 里的那一处不是同一个对象，所以按结构判断，不比引用。
 */
function isFileCredit(parent: Parents) {
	if (!('children' in parent)) return false;
	let label = '';
	let codes = 0;
	for (const child of parent.children) {
		if (child.type === 'text') {
			label += child.value;
			continue;
		}
		if (!isCodeElement(child)) return false;
		codes += 1;
	}
	if (codes !== 1) return false;
	const trimmed = label.trim();
	return trimmed === '源文件：' || trimmed === 'Source:';
}

function isCodeElement(node: Parents['children'][number]) {
	if (node.type === 'element') return node.tagName === 'code';
	if (node.type === 'mdxJsxFlowElement' || node.type === 'mdxJsxTextElement') return node.name === 'code';
	return false;
}

function text(value: string): Text {
	return { type: 'text', value };
}

function hasClass(node: Element, name: string) {
	const className = node.properties?.className;
	if (Array.isArray(className)) return className.includes(name);
	return className === name;
}
