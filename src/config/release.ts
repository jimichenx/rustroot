import fs from 'node:fs';
import path from 'node:path';
import { loadCourse, publicSlug, type LessonNode } from './course-slugs';
import { CURRENT } from './site';

// 正式站的发布范围。只在设置了 RELEASE_POINTER_FILE、且当前是中文构建时生效。
// 预览站、本地 dev、英文构建都不读这个文件，正文全部保留（草稿仍按原规则）。
//
// 文件由正式发布脚本传入，格式见 author/发布指针：
// 主线按阅读顺序只有一个指针，发布到该页为止（含它前面的篇页、章页、节）。
// 附录每一章单独一条线。任一附录页入选时，附录扉页也入选。
// 首页、关于、404 始终发布。
// 已入选的页面若链到未入选的页面，正式构建把链接收成普通文字，源文件不动。

interface Gate {
	allowed: Set<string>;
	known: Set<string>;
}

interface Cache {
	key: string;
	gate: Gate;
}

let cache: Cache | undefined;

function byOrder(a: LessonNode, b: LessonNode): number {
	const ao = a.order ?? Number.POSITIVE_INFINITY;
	const bo = b.order ?? Number.POSITIVE_INFINITY;
	if (ao !== bo) return ao - bo;
	return a.file.localeCompare(b.file);
}

function docsRoot(contentDir: string): string {
	return path.join('src/content/docs', contentDir);
}

function walk(dir: string, rel = ''): string[] {
	if (!fs.existsSync(dir)) return [];
	const out: string[] = [];
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		if (entry.name.startsWith('.') || entry.name.startsWith('_')) continue;
		const childRel = rel ? `${rel}/${entry.name}` : entry.name;
		const child = path.join(dir, entry.name);
		if (entry.isDirectory()) out.push(...walk(child, childRel));
		else if (entry.name.endsWith('.md') || entry.name.endsWith('.mdx')) out.push(childRel);
	}
	return out;
}

function frame(slug: string): boolean {
	return slug === 'index' || slug === '404' || slug === 'about' || slug.startsWith('about/');
}

function readPointers(file: string): string[] {
	if (!fs.existsSync(file)) throw new Error(`找不到发布指针：${file}`);
	const pointers: string[] = [];
	const lines = fs.readFileSync(file, 'utf8').split(/\r?\n/);
	for (let i = 0; i < lines.length; i++) {
		const line = lines[i].trim();
		if (!line || line.startsWith('#')) continue;
		const slug = line.replace(/^\/+|\/+$/g, '');
		if (!/^[a-z0-9/-]+$/.test(slug)) {
			throw new Error(`${file}:${i + 1} 不是公开地址：${line}`);
		}
		pointers.push(slug);
	}
	if (pointers.length === 0) throw new Error(`${file} 里没有发布指针`);
	return pointers;
}

function mainSequence(contentDir: string): string[] {
	const course = loadCourse(contentDir);
	const seq: string[] = [];
	if (course.contents) seq.push(course.contents.slug);
	for (const part of course.parts.filter(part => !part.unnumbered)) {
		if (part.hasIndex) seq.push(part.slug);
		for (const chapter of part.chapters) {
			if (chapter.slug) seq.push(chapter.slug);
			for (const lesson of [...chapter.lessons].sort(byOrder)) seq.push(lesson.slug);
		}
	}
	for (const part of course.parts.filter(part => part.unnumbered)) {
		if (part.hasIndex) seq.push(part.slug);
		for (const lesson of [...part.lessons].sort(byOrder)) seq.push(lesson.slug);
	}
	return seq;
}

function appendixSequences(contentDir: string): Map<string, string[]> {
	const course = loadCourse(contentDir);
	const sequences = new Map<string, string[]>();
	for (const chapter of course.appendix?.chapters ?? []) {
		const seq: string[] = [];
		if (chapter.slug) seq.push(chapter.slug);
		for (const lesson of [...chapter.lessons].sort(byOrder)) seq.push(lesson.slug);
		if (chapter.slug) sequences.set(chapter.slug, seq);
	}
	return sequences;
}

function through(seq: string[], slug: string): string[] {
	const at = seq.indexOf(slug);
	return at < 0 ? [] : seq.slice(0, at + 1);
}

function compute(contentDir: string, file: string): Set<string> {
	const pointers = readPointers(file);
	const main = mainSequence(contentDir);
	const appendix = appendixSequences(contentDir);
	let mainPointer: string | undefined;
	const chapterPointer = new Map<string, string>();

	for (const slug of pointers) {
		if (main.includes(slug)) {
			if (mainPointer) {
				throw new Error(`主线只能有一个发布指针，已有 ${mainPointer}，又写了 ${slug}`);
			}
			mainPointer = slug;
			continue;
		}
		let placed = false;
		for (const [chapter, seq] of appendix) {
			if (!seq.includes(slug)) continue;
			const previous = chapterPointer.get(chapter);
			if (previous) {
				throw new Error(`附录 ${chapter} 只能有一个发布指针，已有 ${previous}，又写了 ${slug}`);
			}
			chapterPointer.set(chapter, slug);
			placed = true;
			break;
		}
		if (!placed) throw new Error(`发布指针 ${slug} 不是主线或附录上的页面`);
	}

	const allowed = new Set<string>();
	if (mainPointer) {
		for (const slug of through(main, mainPointer)) allowed.add(slug);
	}
	let appendixPage = false;
	for (const [chapter, seq] of appendix) {
		const pointer = chapterPointer.get(chapter);
		if (!pointer) continue;
		appendixPage = true;
		for (const slug of through(seq, pointer)) allowed.add(slug);
	}
	if (appendixPage) allowed.add('appendix');

	const known = new Set<string>();
	for (const entry of walk(docsRoot(contentDir))) {
		const slug = publicSlug(contentDir, entry);
		known.add(slug);
		if (frame(slug)) allowed.add(slug);
	}

	console.log(`正式站发布到：${pointers.join('、')}`);
	return { allowed, known };
}

/** 未设置发布指针时返回 null，表示不截断。 */
function releaseGate(): Gate | null {
	const file = process.env.RELEASE_POINTER_FILE;
	if (!file || CURRENT.contentDir !== 'zh-cn') return null;
	if (!fs.existsSync(file)) throw new Error(`找不到发布指针：${file}`);
	const mtimeMs = fs.statSync(file).mtimeMs;
	const key = `${CURRENT.contentDir}\0${file}\0${mtimeMs}`;
	if (cache?.key === key) return cache.gate;
	const gate = compute(CURRENT.contentDir, file);
	cache = { key, gate };
	return gate;
}

/** 这一页是否进入当前这次构建。没有发布指针时，每一页都进。 */
export function isPublished(slug: string): boolean {
	const gate = releaseGate();
	if (!gate) return true;
	return gate.allowed.has(slug);
}

/** 仓库里有这一页，但这一次正式构建不发布它。 */
export function isHeldBack(slug: string): boolean {
	const gate = releaseGate();
	if (!gate) return false;
	return gate.known.has(slug) && !gate.allowed.has(slug);
}
