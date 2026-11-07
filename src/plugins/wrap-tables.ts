import type { Element } from 'hast';
import { defineHastPlugin } from 'satteri';

/** 给 Markdown 表格加一层滚动容器。列宽仍由 table 自己分配。 */
export const wrapTablesPlugin = defineHastPlugin({
	name: 'rustroot-wrap-tables',
	element: {
		filter: ['table'],
		visit(node, ctx) {
			const parent = ctx.parent(node);
			if (parent?.type === 'element' && hasClass(parent, 'table-scroll')) return;
			const wrapper: Element = {
				type: 'element',
				tagName: 'div',
				properties: { className: ['table-scroll'] },
				children: [],
			};
			ctx.wrapNode(node, wrapper);
		},
	},
});

function hasClass(node: Element, name: string) {
	const className = node.properties?.className;
	if (Array.isArray(className)) return className.includes(name);
	return className === name;
}
