import { fileURLToPath } from 'node:url';
import type { Element, Text } from 'hast';
import { defineHastPlugin } from 'satteri';
import { chapterLessonLinks, chapterLessonsHeading } from '../config/course-numbers';
import { CURRENT } from '../config/site';

/**
 * 章描述页末尾补上「目录 / Table of Contents」结构化课表卡片。
 * 课的顺序、课号、是否在生产构建里出现，都跟侧栏同一套；链接文字用课的 title。
 */
export const chapterLessonsPlugin = defineHastPlugin({
	name: 'rustroot-chapter-lessons',
	after(root, ctx) {
		const entry = contentEntry(ctx.fileURL);
		if (!entry) return;
		const lessons = chapterLessonLinks(CURRENT.contentDir, entry);
		if (!lessons?.length) return;

		const heading: Element = {
			type: 'element',
			tagName: 'h2',
			properties: {},
			children: [text(chapterLessonsHeading(CURRENT.contentDir))],
		};
		const list: Element = {
			type: 'element',
			tagName: 'ol',
			properties: { className: ['rr-chapter-lessons'] },
			children: lessons.map((lesson) => {
				const match = lesson.label.match(/^((?:\d+|[A-Z])\.\d+)\s+([\s\S]+)$/);
				const num = match ? match[1] : '';
				const bareTitle = match ? match[2] : lesson.label;
				return {
					type: 'element',
					tagName: 'li',
					properties: { className: ['rr-chapter-lesson'] },
					children: [
						{
							type: 'element',
							tagName: 'a',
							properties: { href: lesson.href, className: ['rr-chapter-lesson-link'] },
							children: [
								...(num
									? [
											{
												type: 'element',
												tagName: 'span',
												properties: {
													className: ['rr-chapter-lesson-num'],
													'aria-hidden': 'true',
												},
												children: [text(num)],
											} satisfies Element,
										]
									: []),
								{
									type: 'element',
									tagName: 'span',
									properties: { className: ['rr-chapter-lesson-title'] },
									children: [text(bareTitle)],
								},
								{
									type: 'element',
									tagName: 'span',
									properties: {
										className: ['rr-chapter-lesson-arrow'],
										'aria-hidden': 'true',
									},
									children: [text('→')],
								},
							],
						},
					],
				};
			}),
		};
		ctx.appendChild(root, [heading, list]);
	},
});

function contentEntry(fileURL: URL | undefined): string | undefined {
	if (!fileURL) return;
	const file = fileURLToPath(fileURL).replace(/\\/g, '/');
	const marker = `/src/content/docs/${CURRENT.contentDir}/`;
	const at = file.lastIndexOf(marker);
	if (at < 0) return;
	return file.slice(at + marker.length);
}

function text(value: string): Text {
	return { type: 'text', value };
}
