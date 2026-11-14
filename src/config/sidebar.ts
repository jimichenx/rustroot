import { compareLessons, courseHeadings } from './course-numbers';
import { loadCourse } from './course-slugs';
import { draftsVisible, isPublished } from './release';
import { CURRENT } from './site';

// 侧栏只看当前语言的 course/。公开地址由 course-slugs 计算，这里只排顺序：
//   篇按其中最小的章号先后。一篇的扉页排在该篇各章之前。
//   章按 chNN 文件夹排序。组标题用章描述页的 title。描述页应显示时，
//   作为组内第一项并带 data-chapter-page，SidebarSublist 据此把组标题
//   渲染成指向章页的链接。
//   课在章内按 sidebar.order 排序。篇、章、课的编号在这里和内容加载时补上，源文件不写。
// 当前语言还没有的页面不出现。章描述页标了草稿，正式构建整章不进侧栏；
// 只标在某一节上时，只扣这一节。dev 和预览构建忽略草稿。章组里没有任何可列出的项时，组不保留。
//
// Unnumbered parts follow numbered parts; the appendix has its own chapter sequence.
// 「开始」不在 course/ 下，仍用 autogenerate。Starlight 用相对
// src/content/docs 的路径匹配 directory，zh-cn/ 与 en/ 不是已配置的 locale，
// 前缀不会被剥掉，所以自动分组必须带上 contentDir。

const auto = (directory: string) => ({
	autogenerate: { directory: `${CURRENT.contentDir}/${directory}` },
});

function isListed(draft: boolean, slug: string): boolean {
	if (!draftsVisible() && draft) return false;
	return isPublished(slug);
}

function chapterFallback(num: number): string {
	return CURRENT.code === 'en' ? `Chapter ${num}` : `第 ${num} 章`;
}

interface CourseLink {
	label: string;
	slug: string;
	attrs?: { 'data-part-page': true };
}

interface CourseGroup {
	label: string;
	items: { slug: string; attrs?: { 'data-chapter-page': true } }[];
}

function scanCourse(): Array<CourseLink | CourseGroup> {
	const course = loadCourse(CURRENT.contentDir);
	const headings = courseHeadings(CURRENT.contentDir);
	const items: Array<CourseLink | CourseGroup> = [];
	if (course.contents && isListed(course.contents.draft, course.contents.slug)) {
		items.push({ label: course.contents.label, slug: course.contents.slug });
	}

	for (const part of course.parts.filter(part => !part.unnumbered)) {
		if (part.hasIndex && part.label && part.entry && isListed(part.draft, part.slug)) {
			items.push({
				label: headings.get(part.entry)?.(part.label) ?? part.label,
				slug: part.slug,
				attrs: { 'data-part-page': true as const },
			});
		}
		for (const chapter of part.chapters) {
			const lessons = chapter.lessons.filter(lesson => isListed(lesson.draft, lesson.slug)).sort(compareLessons);
			const heading = chapter.entry ? headings.get(chapter.entry) : undefined;
			const group = [
				...(chapter.slug && chapter.hasIndex && isListed(chapter.draft, chapter.slug)
					? [{ slug: chapter.slug, attrs: { 'data-chapter-page': true as const } }]
					: []),
				...lessons.map(lesson => ({ slug: lesson.slug })),
			];
			if (group.length === 0) continue;
			items.push({
				label: (chapter.label && heading ? heading(chapter.label) : undefined) ?? chapterFallback(chapter.num),
				items: group,
			});
		}
	}

	for (const part of course.parts.filter(part => part.unnumbered)) {
		if (!part.hasIndex || !part.label || !part.entry || !isListed(part.draft, part.slug)) continue;
		const lessons = part.lessons.filter(lesson => isListed(lesson.draft, lesson.slug)).sort(compareLessons);
		items.push({
			label: part.label,
			items: [
				{ slug: part.slug, attrs: { 'data-chapter-page': true as const } },
				...lessons.map(lesson => ({ slug: lesson.slug })),
			],
		});
	}

	const appendix = course.appendix;
	if (appendix?.hasIndex && appendix.label && isListed(appendix.draft, appendix.slug)) {
		items.push({
			label: appendix.label,
			slug: appendix.slug,
			attrs: { 'data-part-page': true as const },
		});
	}
	for (const chapter of appendix?.chapters ?? []) {
		const heading = chapter.entry ? headings.get(chapter.entry) : undefined;
		const lessons = chapter.lessons.filter(lesson => isListed(lesson.draft, lesson.slug)).sort(compareLessons);
		const group = [
			...(chapter.slug && isListed(chapter.draft, chapter.slug)
				? [{ slug: chapter.slug, attrs: { 'data-chapter-page': true as const } }]
				: []),
			...lessons.map(lesson => ({ slug: lesson.slug })),
		];
		if (group.length === 0) continue;
		items.push({
			label: chapter.label && heading ? heading(chapter.label) : chapter.label ?? chapter.folder,
			items: group,
		});
	}

	return items;
}

export const sidebar = [
	{
		label: '开始',
		translations: { en: 'Start Here' },
		items: [auto('about')],
	},
	...scanCourse(),
];
