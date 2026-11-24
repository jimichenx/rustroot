import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { AstroIntegration } from 'astro';
import { collectOgCards } from '../og/collect';
import { ogPublicPath } from '../og/path';
import { renderOgPng } from '../og/render';

/** 构建结束时，把带 share 字段的页面画成 1200×630 的 PNG，写进当次 dist。 */
export function ogImages(): AstroIntegration {
	return {
		name: 'rustroot-og-images',
		hooks: {
			'astro:build:done': async ({ dir }) => {
				const root = fileURLToPath(dir);
				const cards = collectOgCards();
				for (const card of cards) {
					const rel = ogPublicPath(card).replace(/^\//, '');
					const file = path.join(root, rel);
					await mkdir(path.dirname(file), { recursive: true });
					await writeFile(file, await renderOgPng(card));
				}
				console.log(`专属分享图：${cards.length} 张`);
			},
		},
	};
}
