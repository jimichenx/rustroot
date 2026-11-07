import type { Parent, PhrasingContent, Strong, Text } from 'mdast';
import { defineMdastPlugin, type MdastVisitorContext } from 'satteri';

/**
 * CommonMark 的左右翼规则：粗体以标点结束、后面紧跟汉字（或字母）时，
 * 闭合的 ** 不算右翼；以标点开头、前面紧贴汉字时，开头的 ** 不算左翼。
 * 两种都会把星号留在正文里。课里的「**数组（Array）**解决」就是前一种。
 *
 * 只补解析器留下的这一类 **。已经收成粗体的不动，代码块里的不动，
 * 转义过的 \*\* 也不动。源文件继续写 **，不要改成 <strong>，
 * 也不要在星号和汉字之间加空格。
 */
export const cjkEmphasisPlugin = defineMdastPlugin({
	name: 'rustroot-cjk-emphasis',
	options: { position: true },
	paragraph: repair,
	heading: repair,
	tableCell: repair,
});

type Inline = PhrasingContent;
type Marker = { index: number; at: number };

function repair(node: Parent, ctx: MdastVisitorContext) {
	repairContainer(node, ctx);
}

function repairContainer(node: Parent, ctx: MdastVisitorContext) {
	const children = node.children as Inline[];
	for (const child of children) {
		if (hasInlineChildren(child)) repairContainer(child, ctx);
	}
	const next = repairStrong(children, ctx);
	if (next) ctx.setProperty(node, 'children', next);
}

function hasInlineChildren(node: Inline): node is Parent & Inline {
	return node.type !== 'text' && node.type !== 'inlineCode' && Array.isArray((node as Parent).children);
}

function repairStrong(children: readonly Inline[], ctx: MdastVisitorContext): Inline[] | undefined {
	const pairs = findPairs(children, ctx);
	if (!pairs.length) return;
	pairs.sort((a, b) => b.open.index - a.open.index || b.open.at - a.open.at);
	let current = children.slice();
	for (const pair of pairs) current = applyPair(current, pair);
	return current;
}

function findPairs(children: readonly Inline[], ctx: MdastVisitorContext) {
	const markers = collectMarkers(children, ctx);
	const pairs: { open: Marker; close: Marker }[] = [];
	const stack: Marker[] = [];
	for (const marker of markers) {
		const top = stack.at(-1);
		if (top && hasInner(top, marker, children)) {
			stack.pop();
			if (shouldRepair(top, marker, children)) pairs.push({ open: top, close: marker });
		} else {
			stack.push(marker);
		}
	}
	return pairs;
}

function collectMarkers(children: readonly Inline[], ctx: MdastVisitorContext): Marker[] {
	const markers: Marker[] = [];
	children.forEach((child, index) => {
		if (child.type !== 'text' || !child.value.includes('**')) return;
		const raw = rawSlice(child, ctx);
		let from = 0;
		while (from < child.value.length) {
			const at = child.value.indexOf('**', from);
			if (at < 0) break;
			if (child.value[at - 1] === '*' || child.value[at + 2] === '*') {
				from = at + 1;
				continue;
			}
			if (raw && escapedAt(raw, at)) {
				from = at + 2;
				continue;
			}
			markers.push({ index, at });
			from = at + 2;
		}
	});
	return markers;
}

function hasInner(open: Marker, close: Marker, children: readonly Inline[]) {
	if (close.index < open.index || (close.index === open.index && close.at < open.at + 3)) return false;
	if (close.index === open.index) return true;
	const after = textValue(children[open.index]).slice(open.at + 2);
	const before = textValue(children[close.index]).slice(0, close.at);
	return Boolean(after || before || close.index > open.index + 1);
}

function shouldRepair(open: Marker, close: Marker, children: readonly Inline[]) {
	const beforeOpen = charBefore(open, children);
	const afterOpen = charAfter(open, children);
	const beforeClose = charBefore(close, children);
	const afterClose = charAfter(close, children);
	const openerBlocked = isPunct(afterOpen) && blocksFlank(beforeOpen);
	const closerBlocked = isPunct(beforeClose) && blocksFlank(afterClose);
	return openerBlocked || closerBlocked;
}

/** 翼规则失败的那一侧：紧贴着标点的另一侧是汉字、字母或代码，不是空格也不是标点。 */
function blocksFlank(ch: string) {
	return ch !== '' && !isSpace(ch) && !isPunct(ch);
}

function charBefore(marker: Marker, children: readonly Inline[]) {
	const value = textValue(children[marker.index]);
	if (marker.at > 0) return value[marker.at - 1] ?? '';
	for (let i = marker.index - 1; i >= 0; i--) {
		const ch = edgeChar(children[i], 'end');
		if (ch) return ch;
	}
	return '';
}

function charAfter(marker: Marker, children: readonly Inline[]) {
	const value = textValue(children[marker.index]);
	if (marker.at + 2 < value.length) return value[marker.at + 2] ?? '';
	for (let i = marker.index + 1; i < children.length; i++) {
		const ch = edgeChar(children[i], 'start');
		if (ch) return ch;
	}
	return '';
}

function edgeChar(node: Inline, side: 'start' | 'end') {
	if (node.type === 'text') {
		if (!node.value) return '';
		return side === 'start' ? node.value[0] : node.value[node.value.length - 1];
	}
	if (node.type === 'inlineCode' || node.type === 'strong' || node.type === 'emphasis') return 'x';
	return '';
}

function applyPair(children: readonly Inline[], pair: { open: Marker; close: Marker }): Inline[] {
	const { open, close } = pair;
	const openText = textValue(children[open.index]);
	const closeText = textValue(children[close.index]);
	const inner: Inline[] = [];
	if (open.index === close.index) {
		pushText(inner, openText.slice(open.at + 2, close.at));
	} else {
		pushText(inner, openText.slice(open.at + 2));
		inner.push(...children.slice(open.index + 1, close.index));
		pushText(inner, closeText.slice(0, close.at));
	}
	const strong: Strong = { type: 'strong', children: inner };
	const next: Inline[] = [];
	next.push(...children.slice(0, open.index));
	pushText(next, openText.slice(0, open.at));
	next.push(strong);
	pushText(next, closeText.slice(close.at + 2));
	next.push(...children.slice(close.index + 1));
	return next;
}

function pushText(nodes: Inline[], value: string) {
	if (value) nodes.push({ type: 'text', value });
}

function textValue(node: Inline | undefined) {
	return node?.type === 'text' ? node.value : '';
}

function rawSlice(node: Text, ctx: MdastVisitorContext) {
	const start = node.position?.start.offset;
	const end = node.position?.end.offset;
	if (start == null || end == null) return;
	return ctx.source.slice(start, end);
}

/** value 里下标 at 的字符，在源码里是不是被反斜杠转义的。 */
function escapedAt(raw: string, valueIndex: number) {
	let valueAt = 0;
	for (let i = 0; i < raw.length; i++) {
		if (raw[i] === '\\' && i + 1 < raw.length) {
			if (valueAt === valueIndex) return true;
			valueAt++;
			i++;
			continue;
		}
		if (valueAt === valueIndex) return false;
		valueAt++;
	}
	return false;
}

function isPunct(ch: string) {
	return /^\p{P}$/u.test(ch);
}

function isSpace(ch: string) {
	return /^\s$/u.test(ch);
}
