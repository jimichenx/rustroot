import fs from 'node:fs';
import path from 'node:path';
import { loadCourse, publicSlug, type ChapterNode, type PartNode } from './course-slugs';
import { CURRENT } from './site';

// 正式站的发布范围由正文 frontmatter 的 draft 决定，只在中文生产构建里生效。
// 章描述页 draft: true：这一章整章不进正式站。某一节 draft: true：只扣这一节。
// 一篇里还有未标草稿的章，篇扉页才发布；附录同理。首页、关于、404 始终发布。
// dev 和预览构建（PUBLISH_DRAFTS=1）忽略 draft。已发布页指向草稿页的链接，
// 正式构建收成普通文字，源文件不动。

interface Gate {
	held: Set<string>;
	known: Set<string>;
}

let gateCache: { key: string; gate: Gate } | undefined;

export function draftsVisible(): boolean {
	return (
		process.argv.includes('dev') ||
		process.env.PUBLISH_DRAFTS === '1' ||
		process.env.npm_lifecycle_event === 'dev' ||
		process.env.npm_lifecycle_event === 'dev:en'
	);
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

function holdChapter(chapter: ChapterNode, held: Set<string>, listed: string[]): boolean {
	if (chapter.draft) {
		if (chapter.slug) {
			held.add(chapter.slug);
			listed.push(`${chapter.slug}（整章）`);
		}
		for (const lesson of chapter.lessons) held.add(lesson.slug);
		return false;
	}
	let live = Boolean(chapter.hasIndex && chapter.slug);
	for (const lesson of chapter.lessons) {
		if (!lesson.draft) {
			live = true;
			continue;
		}
		held.add(lesson.slug);
		listed.push(lesson.slug);
	}
	return live;
}

function holdPart(part: PartNode, held: Set<string>, listed: string[]): void {
	let live = false;
	if (part.draft) held.add(part.slug);
	for (const lesson of part.lessons) {
		if (part.draft || lesson.draft) {
			held.add(lesson.slug);
			if (lesson.draft) listed.push(lesson.slug);
		} else {
			live = true;
		}
	}
	for (const chapter of part.chapters) {
		if (holdChapter(chapter, held, listed)) live = true;
	}
	if (part.hasIndex && !live) held.add(part.slug);
}

function compute(contentDir: string): Gate {
	const course = loadCourse(contentDir);
	const held = new Set<string>();
	const known = new Set<string>();
	for (const entry of walk(docsRoot(contentDir))) known.add(publicSlug(contentDir, entry));

	const listed: string[] = [];
	for (const part of course.parts) holdPart(part, held, listed);
	if (course.appendix) holdPart(course.appendix, held, listed);
	if (course.contents?.draft) held.add(course.contents.slug);
	for (const slug of [...held]) {
		if (frame(slug)) held.delete(slug);
	}

	console.log(listed.length === 0 ? '正式站没有排除草稿' : `正式站排除草稿：${listed.join('、')}`);
	return { held, known };
}

/** 未截断时返回 null：dev、预览、英文构建。 */
function releaseGate(): Gate | null {
	if (draftsVisible() || CURRENT.contentDir !== 'zh-cn') return null;
	const key = CURRENT.contentDir;
	if (gateCache?.key === key) return gateCache.gate;
	const gate = compute(key);
	gateCache = { key, gate };
	return gate;
}

/** 这一页是否进入当前这次构建。忽略草稿时，每一页都进。 */
export function isPublished(slug: string): boolean {
	const gate = releaseGate();
	if (!gate) return true;
	return !gate.held.has(slug);
}

/** 仓库里有这一页，但这一次正式构建不发布它。 */
export function isHeldBack(slug: string): boolean {
	const gate = releaseGate();
	if (!gate) return false;
	return gate.known.has(slug) && gate.held.has(slug);
}
