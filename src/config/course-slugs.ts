import fs from 'node:fs';
import path from 'node:path';

// 目录按篇收拢，章用全书连续的 chNN，方便在文件夹里看见顺序：
//   course/index.md                              → /course/
//   course/ownership/index.md                    → /ownership/
//   course/ownership/ch01/index.md（id: bills）   → /ownership/bills/
//   course/ownership/ch01/hard-triangle.mdx      → /ownership/hard-triangle/
// 篇的先后由其中最小的章号决定，所以 types/ 里的 ch04 排在 ownership/ 的 ch03 之后。
// 同一篇里改 chNN，公开地址不变。课换到另一篇，前缀跟着篇文件夹走。
// 篇扉页写 unnumbered: true 时，课直接放在篇文件夹里，不建 chNN，不占篇序和章号。
//
// 英文目录还没有扉页和章描述页时，同一相对路径的课借用中文算出的地址。

const slugText = /^[a-z][a-z0-9-]*$/;
const chapterDirName = /^ch\d{2}$/;
const reserved = new Set(['course', 'about', 'appendix', 'index']);

export interface LessonNode {
	entry: string;
	slug: string;
	/** 课的 title，不带课号。章页课表用它，侧栏用 label。 */
	title: string;
	order?: number;
	draft: boolean;
	file: string;
}

export interface ChapterNode {
	folder: string;
	num: number;
	id?: string;
	slug?: string;
	label?: string;
	draft: boolean;
	hasIndex: boolean;
	entry?: string;
	file?: string;
	lessons: LessonNode[];
}

export interface PartNode {
	slug: string;
	label?: string;
	draft: boolean;
	hasIndex: boolean;
	/** 篇扉页 `unnumbered: true`。不占篇序和章号，课直接放在篇文件夹里。 */
	unnumbered: boolean;
	entry?: string;
	file?: string;
	/** 不编号篇的课。编号篇的课在各章里。 */
	lessons: LessonNode[];
	chapters: ChapterNode[];
}

export interface CourseOutline {
	contents?: { label: string; slug: string; draft: boolean };
	parts: PartNode[];
        appendix?: PartNode;
}

interface PageMeta {
	title?: string;
	label?: string;
	order?: number;
	before?: string;
	id?: string;
	draft: boolean;
	unnumbered: boolean;
}

interface CacheEntry {
	fingerprint: string;
	outline: CourseOutline;
	slugs: Map<string, string>;
}

const cache = new Map<string, CacheEntry>();

function docsRoot(contentDir: string): string {
	return path.join('src/content/docs', contentDir);
}

function contentName(name: string): boolean {
	return !skippedName(name) && (name.endsWith('.md') || name.endsWith('.mdx'));
}

/** 点开头的是系统文件（如 .DS_Store），下划线开头的是私有片段。 */
function skippedName(name: string): boolean {
	return name.startsWith('.') || name.startsWith('_');
}

function stem(name: string): string {
	return name.replace(/\.mdx?$/, '');
}

function defaultSlug(entry: string): string {
	return entry.replace(/\\/g, '/').replace(/\.(md|mdx)$/, '').replace(/\/index$/, '');
}

function unquote(raw: string): string {
	const value = raw.trim();
	if (
		(value.startsWith('"') && value.endsWith('"')) ||
		(value.startsWith("'") && value.endsWith("'"))
	) {
		return value.slice(1, -1);
	}
	return value;
}

function readMeta(file: string): PageMeta {
	const text = fs.readFileSync(file, 'utf8');
	const block = text.match(/^---\r?\n([\s\S]*?)\r?\n---/)?.[1] ?? '';
	const meta: PageMeta = { draft: false, unnumbered: false };
	let inSidebar = false;
	for (const line of block.split('\n')) {
		if (!line.trim() || line.trimStart().startsWith('#')) continue;
		const indent = line.match(/^\s*/)?.[0].length ?? 0;
		const field = line.match(/^\s*([A-Za-z0-9_-]+):\s*(.*)$/);
		if (!field) continue;
		const key = field[1];
		const value = unquote(field[2]);
		if (indent === 0) {
			inSidebar = key === 'sidebar' && value === '';
			if (key === 'title' && value) meta.title = value;
			if (key === 'before' && value) meta.before = value;
			if (key === 'id' && value) meta.id = value;
			if (key === 'draft') meta.draft = value === 'true';
			if (key === 'unnumbered') meta.unnumbered = value === 'true';
			continue;
		}
		if (!inSidebar) continue;
		if (key === 'label' && value) meta.label = value;
		if (key === 'order' && value !== '') {
			const order = Number(value);
			if (Number.isFinite(order)) meta.order = order;
		}
	}
	return meta;
}

function labelOf(meta: PageMeta, file: string): string {
	const label = meta.label || meta.title;
	if (!label) throw new Error(`${file} 缺少 title，侧栏无法命名`);
	return label;
}

function requireSlug(value: string, file: string, field: string): string {
	if (!slugText.test(value)) {
		throw new Error(`${file} 的 ${field}: ${value} 要用小写英文和短横线，如 bills`);
	}
	if (reserved.has(value)) {
		throw new Error(`${file} 的 ${field}: ${value} 与站点路径冲突`);
	}
	return value;
}

function walkFiles(dir: string, rel = ''): string[] {
	if (!fs.existsSync(dir)) return [];
	const out: string[] = [];
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		if (skippedName(entry.name)) continue;
		const childRel = rel ? `${rel}/${entry.name}` : entry.name;
		const child = path.join(dir, entry.name);
		if (entry.isDirectory()) out.push(...walkFiles(child, childRel));
		else if (contentName(entry.name)) out.push(childRel);
	}
	return out;
}

function fingerprint(contentDir: string): string {
	const root = docsRoot(contentDir);
	const own = walkFiles(root)
		.map(entry => {
			const stat = fs.statSync(path.join(root, entry));
			return `${entry}:${stat.mtimeMs}`;
		})
		.join('\n');
	if (contentDir === 'zh-cn') return own;
	return `${own}\n--\n${fingerprint('zh-cn')}`;
}

function rejectBefore(meta: PageMeta, file: string): void {
	if (meta.before) throw new Error(`${file} 不再使用 before。一章属于哪一篇，由它坐在哪个篇文件夹里决定`);
}

/** 只看这一个语言目录。课的地址是「篇文件夹名/文件名」，章的地址是「篇文件夹名/id」。 */
function parseCourse(contentDir: string): { outline: CourseOutline; local: Map<string, string> } {
	const courseDir = path.join(docsRoot(contentDir), 'course');
	const outline: CourseOutline = { parts: [] };
	const local = new Map<string, string>();
	if (!fs.existsSync(courseDir)) return { outline, local };

	const dirents = fs.readdirSync(courseDir, { withFileTypes: true }).filter(entry => !skippedName(entry.name));
	const indexFiles = dirents.filter(entry => entry.isFile() && /^index\.mdx?$/.test(entry.name));
	if (indexFiles.length > 1) throw new Error(`${courseDir} 里有多份课程目录 index`);
	if (indexFiles[0]) {
		const file = path.join(courseDir, indexFiles[0].name);
		const meta = readMeta(file);
		rejectBefore(meta, file);
		if (meta.id) throw new Error(`${file} 是课程目录，公开地址固定为 /course/`);
		outline.contents = { label: labelOf(meta, file), slug: 'course', draft: meta.draft };
	}
	for (const entry of dirents) {
		if (!entry.isFile() || /^index\.mdx?$/.test(entry.name)) continue;
		throw new Error(`${path.join(courseDir, entry.name)} 不应直接放在 course/ 下。篇扉页是 course/<篇>/index.md`);
	}

	const seenChapters = new Map<string, string>();
	const seenIds = new Map<string, string>();
	for (const entry of dirents) {
		if (!entry.isDirectory()) continue;
		if (chapterDirName.test(entry.name)) {
			throw new Error(`${path.join(courseDir, entry.name)} 要放进篇文件夹，如 course/ownership/ch01`);
		}
		const partSlug = requireSlug(entry.name, path.join(courseDir, entry.name), '篇文件夹');
		const partDir = path.join(courseDir, entry.name);
		const partEntries = fs.readdirSync(partDir, { withFileTypes: true }).filter(item => !skippedName(item.name));
		const partIndex = partEntries.find(item => item.isFile() && /^index\.mdx?$/.test(item.name));
		const part: PartNode = {
			slug: partSlug,
			draft: false,
			hasIndex: false,
			unnumbered: false,
			lessons: [],
			chapters: [],
		};
		if (partIndex) {
			const file = path.join(partDir, partIndex.name);
			const meta = readMeta(file);
			rejectBefore(meta, file);
			if (meta.id) throw new Error(`${file} 是篇扉页，公开地址就是文件夹名 /${partSlug}/`);
			part.label = labelOf(meta, file);
			part.draft = meta.draft;
			part.unnumbered = meta.unnumbered;
			part.hasIndex = true;
			part.entry = `course/${entry.name}/${partIndex.name}`;
			part.file = file;
			local.set(part.entry, partSlug);
		}

		for (const item of partEntries) {
			if (item.isFile() && !/^index\.mdx?$/.test(item.name)) {
				if (!part.unnumbered || !contentName(item.name)) {
					throw new Error(`${path.join(partDir, item.name)} 要放进章文件夹，如 ${entry.name}/ch01/`);
				}
				const full = path.join(partDir, item.name);
				const meta = readMeta(full);
				rejectBefore(meta, full);
				if (meta.id) throw new Error(`${full} 的公开地址由篇文件夹和文件名决定，不要写 id`);
				const lessonEntry = `course/${entry.name}/${item.name}`;
				const slug = `${partSlug}/${stem(item.name)}`;
				local.set(lessonEntry, slug);
				part.lessons.push({
					entry: lessonEntry,
					slug,
					title: meta.title || meta.label || stem(item.name),
					order: meta.order,
					draft: meta.draft,
					file: full,
				});
				continue;
			}
			if (!item.isDirectory()) continue;
			if (part.unnumbered) {
				throw new Error(`${path.join(partDir, item.name)} 属于不编号的篇，课直接放在篇文件夹里，不要再建章`);
			}
			if (!chapterDirName.test(item.name)) {
				throw new Error(`${path.join(partDir, item.name)} 不是章文件夹。章文件夹用两位编号，如 ch01`);
			}
			const previous = seenChapters.get(item.name);
			if (previous) throw new Error(`${item.name} 同时出现在 ${previous} 和 ${entry.name}。章号全书连续，不能重复`);
			seenChapters.set(item.name, entry.name);

			const dir = path.join(partDir, item.name);
			const files = fs.readdirSync(dir).filter(contentName);
			const indexNames = files.filter(file => /^index\.mdx?$/.test(file));
			if (indexNames.length > 1) throw new Error(`${dir} 里有多份章描述页 index`);
			const indexName = indexNames[0];
			const indexPath = indexName ? path.join(dir, indexName) : undefined;
			const indexMeta = indexPath ? readMeta(indexPath) : undefined;
			if (indexMeta) rejectBefore(indexMeta, indexPath!);
			if (indexPath && !indexMeta?.id) {
				throw new Error(`${indexPath} 需要 id，作为这一章的公开地址，例如 id: bills`);
			}
			const id = indexMeta?.id ? requireSlug(indexMeta.id, indexPath!, 'id') : undefined;
			if (id) {
				const idOwner = seenIds.get(id);
				if (idOwner) throw new Error(`${indexPath} 的 id: ${id} 与 ${idOwner} 重复`);
				seenIds.set(id, indexPath!);
			}
			const lessons: LessonNode[] = files
				.filter(file => !/^index\.mdx?$/.test(file))
				.map(file => {
					const full = path.join(dir, file);
					const meta = readMeta(full);
					rejectBefore(meta, full);
					if (meta.id) throw new Error(`${full} 的公开地址由篇文件夹和文件名决定，不要写 id`);
					const lessonEntry = `course/${entry.name}/${item.name}/${file}`;
					const slug = `${partSlug}/${stem(file)}`;
					local.set(lessonEntry, slug);
					return {
						entry: lessonEntry,
						slug,
						title: meta.title || meta.label || stem(file),
						order: meta.order,
						draft: meta.draft,
						file: full,
					};
				});
			const chapterEntry = indexName ? `course/${entry.name}/${item.name}/${indexName}` : undefined;
			const chapterSlug = id ? `${partSlug}/${id}` : undefined;
			if (chapterEntry && chapterSlug) local.set(chapterEntry, chapterSlug);
			part.chapters.push({
				folder: item.name,
				num: Number(item.name.slice(2)),
				id,
				slug: chapterSlug,
				label: indexMeta ? labelOf(indexMeta, indexPath!) : undefined,
				draft: indexMeta?.draft ?? false,
				hasIndex: Boolean(indexPath),
				entry: chapterEntry,
				file: indexPath,
				lessons,
			});
		}
		part.chapters.sort((a, b) => a.folder.localeCompare(b.folder));
		outline.parts.push(part);
	}

	outline.parts.sort((a, b) => {
		const an = a.chapters[0]?.num ?? Number.POSITIVE_INFINITY;
		const bn = b.chapters[0]?.num ?? Number.POSITIVE_INFINITY;
		if (an !== bn) return an - bn;
		return a.slug.localeCompare(b.slug);
	});
	return { outline, local };
}

function parseAppendix(contentDir: string): PartNode | undefined {
        const dir = path.join(docsRoot(contentDir), 'appendix');
        if (!fs.existsSync(dir)) return undefined;

        const index = path.join(dir, 'index.md');
        if (!fs.existsSync(index)) throw new Error(`${dir} 缺少附录扉页 index.md`);
        const meta = readMeta(index);
        const appendix: PartNode = {
                slug: 'appendix',
                label: labelOf(meta, index),
                draft: meta.draft,
                hasIndex: true,
                unnumbered: true,
                entry: 'appendix/index.md',
                file: index,
                lessons: [],
                chapters: [],
        };
        const seenOrders = new Set<number>();
        for (const item of fs.readdirSync(dir, { withFileTypes: true })) {
                if (skippedName(item.name) || item.name === 'index.md') continue;
                if (!item.isDirectory()) throw new Error(`${path.join(dir, item.name)} 要放进附录章文件夹`);
                const chapterDir = path.join(dir, requireSlug(item.name, path.join(dir, item.name), '附录章文件夹'));
                const chapterIndex = path.join(chapterDir, 'index.md');
                if (!fs.existsSync(chapterIndex)) throw new Error(`${chapterDir} 缺少章描述页 index.md`);
                const chapterMeta = readMeta(chapterIndex);
                const order = chapterMeta.order;
                if (!Number.isInteger(order) || order! < 1 || order! > 26 || seenOrders.has(order!)) {
                        throw new Error(`${chapterIndex} 的 sidebar.order 应为不重复的 1～26 整数`);
                }
                seenOrders.add(order!);
                const slug = `appendix/${item.name}`;
                const lessons = fs.readdirSync(chapterDir)
                        .filter(file => contentName(file) && !/^index\.mdx?$/.test(file))
                        .map(file => {
                                const full = path.join(chapterDir, file);
                                const lessonMeta = readMeta(full);
                                return {
                                        entry: `${slug}/${file}`,
                                        slug: `${slug}/${stem(file)}`,
                                        title: lessonMeta.title || lessonMeta.label || stem(file),
                                        order: lessonMeta.order,
                                        draft: lessonMeta.draft,
                                        file: full,
                                };
                        });
                appendix.chapters.push({
                        folder: item.name,
                        num: order!,
                        slug,
                        label: labelOf(chapterMeta, chapterIndex),
                        draft: chapterMeta.draft,
                        hasIndex: true,
                        entry: `${slug}/index.md`,
                        file: chapterIndex,
                        lessons,
                });
        }
        appendix.chapters.sort((a, b) => a.num - b.num);
        return appendix;
}

function resolved(contentDir: string): CacheEntry {
	const fp = fingerprint(contentDir);
	const hit = cache.get(contentDir);
	if (hit?.fingerprint === fp) return hit;

	const { outline, local } = parseCourse(contentDir);
        outline.appendix = parseAppendix(contentDir);
	const zh = contentDir === 'zh-cn' ? local : parseCourse('zh-cn').local;
	const slugs = new Map<string, string>();
	for (const entry of walkFiles(docsRoot(contentDir))) {
		const slug = local.get(entry) ?? (contentDir === 'zh-cn' ? undefined : zh.get(entry)) ?? defaultSlug(entry);
		const previous = [...slugs.entries()].find(([, taken]) => taken === slug);
		if (previous) throw new Error(`${entry} 的公开地址 /${slug}/ 与 ${previous[0]} 冲突`);
		slugs.set(entry, slug);
	}
	for (const part of outline.parts) {
		for (const lesson of part.lessons) {
			lesson.slug = slugs.get(lesson.entry) ?? lesson.slug;
		}
		for (const chapter of part.chapters) {
			for (const lesson of chapter.lessons) {
				lesson.slug = slugs.get(lesson.entry) ?? lesson.slug;
			}
		}
	}
	assertLinks(contentDir, new Set(slugs.values()));
	const entry = { fingerprint: fp, outline, slugs };
	cache.set(contentDir, entry);
	return entry;
}

function assertLinks(contentDir: string, slugs: Set<string>): void {
	const root = docsRoot(contentDir);
	const broken: string[] = [];
	const pattern = /\]\((\/[^)\s#]+)|href=["'](\/[^"'#]+)|link:\s*(\/[^\s#]+)/g;
	for (const entry of walkFiles(root)) {
		const text = fs.readFileSync(path.join(root, entry), 'utf8');
		for (const match of text.matchAll(pattern)) {
			const href = match[1] || match[2] || match[3];
			const target = href.replace(/^\//, '').replace(/\/$/, '');
			if (target === '' || slugs.has(target)) continue;
			broken.push(`${entry} → ${href}`);
		}
	}
	if (broken.length > 0) {
		throw new Error(`站内链接指向不存在的页面：\n${broken.join('\n')}`);
	}
}

export function loadCourse(contentDir: string): CourseOutline {
	return resolved(contentDir).outline;
}

/** 内容集合的 id，也就是去掉首尾斜杠的公开路径。 */
export function publicSlug(contentDir: string, entry: string): string {
	const normalized = entry.replace(/\\/g, '/');
	return resolved(contentDir).slugs.get(normalized) ?? defaultSlug(normalized);
}
