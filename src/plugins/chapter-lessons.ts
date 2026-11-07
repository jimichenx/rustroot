import { fileURLToPath } from 'node:url';
import type { Element, Text } from 'hast';
import { defineHastPlugin } from 'satteri';
import { chapterLessonLinks, chapterLessonsHeading } from '../config/course-numbers';
import { CURRENT } from '../config/site';

/**
 * 章描述页末尾补上「本章课」。
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
			tagName: 'ul',
			properties: {},
			children: lessons.map((lesson) => ({
				type: 'element',
				tagName: 'li',
				properties: {},
				children: [
					{
						type: 'element',
						tagName: 'a',
						properties: { href: lesson.href },
						children: [text(lesson.label)],
					},
				],
			})),
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
