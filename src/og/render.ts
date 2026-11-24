import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import satori from 'satori';
import sharp from 'sharp';
import type { OgCard } from './collect';

const WIDTH = 1200;
const HEIGHT = 630;
const FONT = 'Noto Sans SC';

type Node = {
	type: string;
	props: Record<string, unknown>;
};

const fontsDir = fileURLToPath(new URL('../../scripts/brand/fonts/', import.meta.url));
const siteTitlePath = fileURLToPath(
	new URL('../components/starlight/SiteTitle.astro', import.meta.url),
);

let fonts: { name: string; data: Buffer; weight: 400 | 700; style: 'normal' }[] | undefined;
let lockup: { mark: string; wordmark: string } | undefined;

function loadFonts() {
	if (fonts) return fonts;
	const files: [string, 400 | 700][] = [
		['noto-sans-sc-400.woff', 400],
		['noto-sans-sc-700.woff', 700],
		['noto-sans-sc-latin-400.woff', 400],
		['noto-sans-sc-latin-700.woff', 700],
		['noto-sans-sc-latin-ext-400.woff', 400],
		['noto-sans-sc-latin-ext-700.woff', 700],
	];
	fonts = files.map(([name, weight]) => ({
		name: FONT,
		data: fs.readFileSync(path.join(fontsDir, name)),
		weight,
		style: 'normal' as const,
	}));
	return fonts;
}

function svgDataUri(svg: string): string {
	const withNs = svg.includes('xmlns=')
		? svg
		: svg.replace('<svg ', '<svg xmlns="http://www.w3.org/2000/svg" ');
	return `data:image/svg+xml;base64,${Buffer.from(withNs).toString('base64')}`;
}

function loadLockup() {
	if (lockup) return lockup;
	const source = fs.readFileSync(siteTitlePath, 'utf8');
	const mark = source.match(/<svg class="mark"[\s\S]*?<\/svg>/)?.[0]?.replace(/ class="mark"/, '');
	const wordmark = source
		.match(/<svg class="wordmark"[\s\S]*?<\/svg>/)?.[0]
		?.replace(/ class="wordmark"/, '');
	if (!mark || !wordmark) throw new Error('SiteTitle.astro 里没有锁扣 SVG');
	lockup = {
		mark: svgDataUri(mark.replaceAll('currentColor', '#ffffff')),
		wordmark: svgDataUri(wordmark.replaceAll('currentColor', '#ffffff')),
	};
	return lockup;
}

const rings = svgDataUri(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" fill="none">
	<path d="M20 6H44C51.732 6 58 12.268 58 20V23H36.76L24.76 35H6V20C6 12.268 12.268 6 20 6Z" fill="#262932"/>
	<path d="M39.24 29H58V44C58 51.732 51.732 58 44 58H20C12.268 58 6 51.732 6 44V41H27.24L39.24 29Z" fill="#5c2a12"/>
</svg>`);

type Atom = { text: string; em: number };

const cjk = /[\u3400-\u9fff\u3000-\u303f\uff00-\uffef]/;

function emOf(text: string): number {
	let em = 0;
	for (const ch of text) {
		if (cjk.test(ch)) em += 1;
		else if (ch === ' ') em += 0.33;
		else em += 0.58;
	}
	return em;
}

/** 反引号是代码标记。中文与标识符之间补空格，并优先在标点后断行。 */
function atoms(text: string): Atom[] {
	const parts = text.split('`');
	const out: Atom[] = [];
	const push = (value: string) => {
		if (!value) return;
		const prev = out[out.length - 1]?.text ?? '';
		const lead = value[0] ?? '';
		let piece = value;
		const gap =
			!piece.startsWith(' ') &&
			!prev.endsWith(' ') &&
			((cjk.test(prev) && !/、$/.test(prev) && /^[A-Za-z0-9&<]/.test(piece)) ||
				(/[A-Za-z0-9>&]$/.test(prev) && cjk.test(lead) && !/[，。、；：]/.test(lead)));
		if (gap) piece = ` ${piece}`;
		out.push({ text: piece, em: emOf(piece) });
	};
	for (let i = 0; i < parts.length; i++) {
		const piece = parts[i] ?? '';
		if (!piece) continue;
		if (i % 2 === 1) {
			push(piece);
			continue;
		}
		let buf = '';
		for (const ch of piece) {
			buf += ch;
			if (/[，。、；：]/.test(ch)) {
				push(buf);
				buf = '';
			}
		}
		push(buf);
	}
	return out;
}

/** 一条子句比行宽还长时，按字切开，标点留在上一行末尾。 */
function piecesOf(text: string, em: number, maxEm: number): string[] {
	if (em <= maxEm) return [text];
	const rows: string[] = [];
	let row = '';
	let width = 0;
	for (const ch of text) {
		const w = emOf(ch);
		const punct = /[，。、；：]/.test(ch);
		if (row && width + w > maxEm && !punct) {
			rows.push(row);
			row = '';
			width = 0;
		}
		row += ch;
		width += w;
	}
	if (row) rows.push(row);
	return rows;
}

function linesOf(text: string, maxEm: number): string[] {
	const rows: string[] = [];
	let row = '';
	let em = 0;
	const flush = () => {
		if (!row) return;
		rows.push(row);
		row = '';
		em = 0;
	};
	for (const atom of atoms(text)) {
		if (atom.em > maxEm) {
			flush();
			rows.push(...piecesOf(atom.text, atom.em, maxEm));
			continue;
		}
		if (row && em + atom.em > maxEm) flush();
		row += atom.text;
		em += atom.em;
	}
	flush();
	return rows;
}

function textBlock(
	text: string,
	style: Record<string, string | number>,
	maxEm: number,
): Node {
	return el(
		'div',
		{ display: 'flex', flexDirection: 'column', maxWidth: 740 },
		linesOf(text, maxEm).map((line) =>
			el('div', { display: 'flex', whiteSpace: 'nowrap', ...style }, line),
		),
	);
}

function el(
	type: string,
	style: Record<string, string | number>,
	children?: Node | Node[] | string,
	extra?: Record<string, unknown>,
): Node {
	return { type, props: { style, children, ...extra } };
}

function cardTree(card: OgCard): Node {
	const { mark, wordmark } = loadLockup();
	return el(
		'div',
		{
			width: WIDTH,
			height: HEIGHT,
			display: 'flex',
			position: 'relative',
			background: '#17181c',
			fontFamily: FONT,
		},
		[
			el('img', { position: 'absolute', right: -190, top: -35, width: 700, height: 700 }, undefined, {
				src: rings,
			}),
			el(
				'div',
				{
					display: 'flex',
					flexDirection: 'column',
					position: 'relative',
					width: 760,
					marginLeft: 96,
					marginTop: 92,
				},
				[
					el('div', { display: 'flex', flexDirection: 'row', alignItems: 'center' }, [
						el('img', { width: 72, height: 72 }, undefined, { src: mark }),
						el('img', { width: 236, height: 40, marginLeft: 18 }, undefined, { src: wordmark }),
					]),
					el('div', { display: 'flex', marginTop: 52 }, [
						textBlock(
							card.title,
							{ fontSize: 24, lineHeight: 1.45, color: '#9aa0aa' },
							30,
						),
					]),
					el('div', { display: 'flex', marginTop: 22 }, [
						textBlock(
							card.share,
							{ fontSize: 42, fontWeight: 700, lineHeight: 1.38, color: '#ffffff' },
							17.2,
						),
					]),
				],
			),
			el('div', {
				position: 'absolute',
				left: 96,
				bottom: 108,
				width: 1008,
				height: 1,
				background: '#353841',
			}),
			el(
				'div',
				{
					position: 'absolute',
					left: 96,
					bottom: 58,
					fontSize: 18,
					letterSpacing: 1,
					color: '#c0c2c7',
				},
				card.domain,
			),
		],
	);
}

export async function renderOgPng(card: OgCard): Promise<Buffer> {
	const svg = await satori(cardTree(card), {
		width: WIDTH,
		height: HEIGHT,
		fonts: loadFonts(),
		onStyleError(error: Error) {
			throw error;
		},
	});
	return sharp(Buffer.from(svg)).png().toBuffer();
}
