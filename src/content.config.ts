import { defineCollection } from 'astro:content';
import { glob, type Loader } from 'astro/loaders';
import { z } from 'astro/zod';
import { docsSchema, i18nSchema } from '@astrojs/starlight/schema';
import { i18nLoader } from '@astrojs/starlight/loaders';
import { courseHeadings } from './config/course-numbers';
import { publicSlug } from './config/course-slugs';
import { CURRENT } from './config/site';

/** 篇、章、课的标题和侧栏标签在加载后补编号。源文件只写名字。 */
function withCourseNumbers(loader: Loader): Loader {
	return {
		name: 'course-numbered-glob',
		load: async (context) => {
			await loader.load(context);
			const headings = courseHeadings(CURRENT.contentDir);
			const prefix = `src/content/docs/${CURRENT.contentDir}/`;
                        for (const [id, entry] of [...context.store.entries()]) {
				const filePath = entry.filePath?.replace(/\\/g, '/');
				if (!filePath?.startsWith(prefix)) continue;
				const heading = headings.get(filePath.slice(prefix.length));
				if (!heading) continue;
				const data = entry.data as { title?: string; sidebar?: { label?: string } };
                                const title = typeof data.title === 'string' ? heading(data.title) : data.title;
                                const label = typeof data.sidebar?.label === 'string'
                                        ? heading(data.sidebar.label)
                                        : data.sidebar?.label;
                                if (title === data.title && label === data.sidebar?.label) continue;
                                const updated = {
                                        ...data,
                                        title,
                                        ...(data.sidebar ? { sidebar: { ...data.sidebar, label } } : {}),
                                };
                                // Reinsert the entry so Astro persists the change even when its digest is unchanged.
                                context.store.delete(id);
                                context.store.set({ ...entry, id, data: updated });
			}
		},
	};
}

/**
 * 只加载当前语言的内容目录，并把它当作站点根。
 *
 * Starlight 默认的 docsLoader 会加载整个 `src/content/docs/`，
 * 这里改为 `src/content/docs/<contentDir>/`，于是：
 *   src/content/docs/zh-cn/course/index.md  →  https://rustroot.cn/course/
 *   src/content/docs/en/course/index.md     →  https://rustroot.dev/course/
 *
 * course/ 下的课和章不沿用文件路径做地址，见 src/config/course-slugs.ts。
 *
 * pattern 与 Starlight 保持一致：忽略以 `_` 开头的文件（可用于放模板、草稿片段）。
 */
export const collections = {
	docs: defineCollection({
		loader: withCourseNumbers(
			glob({
				base: `./src/content/docs/${CURRENT.contentDir}`,
				pattern: '**/[^_]*.{md,mdx}',
				generateId: ({ entry }) => publicSlug(CURRENT.contentDir, entry),
			}),
		),
		schema: docsSchema({
			// id：章描述页在所属篇下的地址，如 bills → /ownership/bills/。
			extend: z.object({
				id: z.string().regex(/^[a-z][a-z0-9-]*$/).optional(),
				// 篇扉页。为 true 时这一篇不占篇序和章号，课直接放在篇文件夹里。
				unnumbered: z.boolean().optional(),
			}),
		}),
	}),
	/**
	 * Starlight UI 文案覆盖（可选）。
	 * 在 src/content/i18n/<lang>.json 中可以自定义或覆盖内建翻译，例如把 "On this page" 改成更贴合的中文。
	 * 目前为空，仅注册集合以避免构建警告。
	 */
	i18n: defineCollection({ loader: i18nLoader(), schema: i18nSchema() }),
};
