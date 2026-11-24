import fs from 'node:fs';
import path from 'node:path';
import { parse } from 'yaml';
import { courseHeadings } from '../config/course-numbers';
import { publicSlug } from '../config/course-slugs';
import { isPublished } from '../config/release';
import { CURRENT } from '../config/site';

export interface OgCard {
	slug: string;
	title: string;
	share: string;
	domain: string;
}

function walk(dir: string, rel = ''): string[] {
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

function frontmatter(text: string): { title?: unknown; share?: unknown } {
	const match = text.match(/^---\r?\n([\s\S]*?)\r?\n---/);
	if (!match?.[1]) return {};
	const data = parse(match[1]);
	if (!data || typeof data !== 'object') return {};
	return data as { title?: unknown; share?: unknown };
}

/** 有 share 且这次构建会发布的页面。草稿在正式构建里不进。 */
export function collectOgCards(): OgCard[] {
	const contentDir = CURRENT.contentDir;
	const root = path.join(process.cwd(), 'src/content/docs', contentDir);
	if (!fs.existsSync(root)) return [];
	const headings = courseHeadings(contentDir);
	const domain = new URL(CURRENT.site).host;
	const cards: OgCard[] = [];
	for (const rel of walk(root)) {
		const data = frontmatter(fs.readFileSync(path.join(root, rel), 'utf8'));
		const share = typeof data.share === 'string' ? data.share.trim() : '';
		if (!share) continue;
		const slug = publicSlug(contentDir, rel);
		if (!isPublished(slug)) continue;
		const bare = typeof data.title === 'string' ? data.title.trim() : slug;
		const title = headings.get(rel)?.(bare) ?? bare;
		cards.push({ slug, title, share, domain });
	}
	cards.sort((a, b) => a.slug.localeCompare(b.slug));
	return cards;
}
