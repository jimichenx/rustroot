import type { Link } from 'mdast';
import { defineMdastPlugin } from 'satteri';
import { isHeldBack } from '../config/release';

// 正式构建里，指向尚未发布页面的站内链接收成普通文字，避免读者点进 404。
// 源文件不改。预览站和 dev 没有发布指针，链接保持原样。

export const unpublishedLinkPlugin = defineMdastPlugin({
	name: 'rustroot-unpublished-links',
	link(node: Link, ctx) {
		const slug = contentSlug(node.url);
		if (!slug || !isHeldBack(slug)) return;
		ctx.replaceNode(node, node.children);
	},
});

function contentSlug(url: string): string | undefined {
	if (!url.startsWith('/')) return;
	const path = url.split(/[?#]/, 1)[0] ?? '';
	const slug = path.replace(/^\/+|\/+$/g, '');
	return slug || undefined;
}
