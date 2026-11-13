const ANCHOR_START_RE = /^\s*\/\/\s*ANCHOR:\s*([A-Za-z0-9_-]+)\s*$/;
const ANCHOR_END_RE = /^\s*\/\/\s*ANCHOR_END:\s*([A-Za-z0-9_-]+)\s*$/;

/**
 * 支持 `<Code code={src} lang="rust" anchor="name" />`（多段用逗号 `anchor="a,b"` 或数组）
 * 以及围栏 `anchor=name` 提取 `// ANCHOR: name` ～ `// ANCHOR_END: name` 之间的代码。
 * 同时对所有代码块自动剥离残留的 `// ANCHOR` 与 `// ANCHOR_END` 标记行，
 * 避免标记泄漏到页面渲染、复制内容或 Rust Playground。
 */
export const anchorPlugin = {
	name: 'Code Anchor',
	hooks: {
		preprocessMetadata({ codeBlock }) {
			const fromMeta = codeBlock.metaOptions.getString('anchor');
			if (fromMeta !== undefined) {
				codeBlock.props.anchor = fromMeta;
			}
		},
		preprocessCode({ codeBlock }) {
			const requested = parseAnchors(codeBlock.props.anchor);
			const lines = codeBlock.getLines().map((line) => line.text);

			if (requested.length > 0) {
				const segments = requested.map((name) => extractAnchor(lines, name, codeBlock));
				const nextLines = [];
				segments.forEach((seg, idx) => {
					if (idx > 0) nextLines.push('');
					nextLines.push(...seg);
				});
				replaceAllLines(codeBlock, nextLines);
				return;
			}

			// 未指定 anchor 时，剥离源码中嵌套的 // ANCHOR 与 // ANCHOR_END 行
			const indicesToDelete = [];
			lines.forEach((text, idx) => {
				if (ANCHOR_START_RE.test(text) || ANCHOR_END_RE.test(text)) {
					indicesToDelete.push(idx);
				}
			});
			if (indicesToDelete.length > 0) {
				codeBlock.deleteLines(indicesToDelete);
			}
		},
	},
};

function parseAnchors(raw) {
	if (raw === undefined || raw === null || raw === '') return [];
	const list = Array.isArray(raw) ? raw : String(raw).split(',');
	return list.map((s) => String(s).trim()).filter(Boolean);
}

function extractAnchor(lines, name, codeBlock) {
	let startIdx = -1;
	let endIdx = -1;

	for (let i = 0; i < lines.length; i++) {
		const startMatch = lines[i].match(ANCHOR_START_RE);
		if (startMatch && startMatch[1] === name) {
			startIdx = i + 1;
			continue;
		}
		if (startIdx !== -1) {
			const endMatch = lines[i].match(ANCHOR_END_RE);
			if (endMatch && endMatch[1] === name) {
				endIdx = i;
				break;
			}
		}
	}

	const docPath = codeBlock.parentDocument?.sourceFilePath ?? '<unknown>';
	if (startIdx === -1) {
		throw new Error(`代码块未找到起始标记 "// ANCHOR: ${name}"（位于 ${docPath}）。`);
	}
	if (endIdx === -1) {
		throw new Error(`代码块未找到结束标记 "// ANCHOR_END: ${name}"（位于 ${docPath}）。`);
	}

	const extracted = lines
		.slice(startIdx, endIdx)
		.filter((line) => !ANCHOR_START_RE.test(line) && !ANCHOR_END_RE.test(line));

	return dedent(extracted);
}

function dedent(lines) {
	let minIndent = Infinity;
	for (const line of lines) {
		if (!line.trim()) continue;
		const match = line.match(/^[ \t]*/);
		const indent = match ? match[0].length : 0;
		if (indent < minIndent) minIndent = indent;
	}
	if (!Number.isFinite(minIndent) || minIndent === 0) return lines;
	return lines.map((line) => (line.trim() ? line.slice(minIndent) : ''));
}

function replaceAllLines(codeBlock, nextLines) {
	const count = codeBlock.getLines().length;
	if (count > 0) {
		codeBlock.deleteLines(Array.from({ length: count }, (_, i) => i));
	}
	if (nextLines.length > 0) {
		codeBlock.insertLines(0, nextLines);
	}
}
