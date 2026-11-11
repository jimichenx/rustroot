import { loadCourse, type LessonNode } from './course-slugs';
import { isPublished } from './release';

// 篇、章、课的标题和侧栏标签不在源文件里写编号。
// 章号来自 chNN。课号是「章号.章内序号」，序号按 sidebar.order 排好后从 1 起，草稿也占号。
// 篇序是篇在全书里的位置，按其中最小的章号排列。

export type CourseLocale = 'zh' | 'en';

const zhPart = ['', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十'];

export function courseLocale(contentDir: string): CourseLocale {
	return contentDir === 'en' ? 'en' : 'zh';
}

export function compareLessons(a: LessonNode, b: LessonNode): number {
	const ao = a.order ?? Number.POSITIVE_INFINITY;
	const bo = b.order ?? Number.POSITIVE_INFINITY;
	if (ao !== bo) return ao - bo;
	return a.file.localeCompare(b.file);
}

function stripLesson(text: string): string {
        return text.replace(/^(?:\d+|[A-Z])\.\d+\s+/, '');
}

function stripChapter(locale: CourseLocale, text: string): string {
	if (locale === 'zh') return text.replace(/^第\s*\d+\s*章[　\s]+/, '');
	return text.replace(/^Chapter\s+\d+\.\s+/, '');
}

function stripPart(locale: CourseLocale, text: string): string {
	if (locale === 'zh') return text.replace(/^第[一二三四五六七八九十]+篇[　\s]+/, '');
	return text.replace(/^Part\s+\d+\.\s+/, '');
}

export function partHeading(locale: CourseLocale, index: number, text: string): string {
	const bare = stripPart(locale, text);
	if (locale === 'zh') {
		const n = index >= 1 && index <= 10 ? zhPart[index] : String(index);
		return `第${n}篇　${bare}`;
	}
	return `Part ${index}. ${bare}`;
}

export function chapterHeading(locale: CourseLocale, num: number, text: string): string {
	const bare = stripChapter(locale, text);
	if (locale === 'zh') return `第 ${num} 章　${bare}`;
	return `Chapter ${num}. ${bare}`;
}

export function lessonHeading(chapter: number, index: number, text: string): string {
	return `${chapter}.${index} ${stripLesson(text)}`;
}

function appendixLetter(num: number): string {
        return String.fromCharCode(64 + num);
}

export function appendixChapterHeading(locale: CourseLocale, num: number, text: string): string {
        const prefix = locale === 'zh' ? '附录' : 'Appendix';
        const bare = locale === 'zh'
                ? text.replace(/^(?:附录\s+[A-Z]\s+)+/, '')
                : text.replace(/^(?:Appendix\s+[A-Z]\s+)+/, '');
        return `${prefix} ${appendixLetter(num)}　${bare}`;
}

export function chapterLessonsHeading(contentDir: string): string {
	return courseLocale(contentDir) === 'zh' ? '目录' : 'Table of Contents';
}

/** 章描述页末尾的课表。课号与侧栏相同，草稿占号；生产构建不列出草稿。 */
export function chapterLessonLinks(
	contentDir: string,
	chapterEntry: string,
): { href: string; label: string }[] | undefined {
	const showDrafts = process.argv.includes('dev');
	const course = loadCourse(contentDir);
	const headings = courseHeadings(contentDir);
        for (const part of [...course.parts, ...(course.appendix ? [course.appendix] : [])]) {
		for (const chapter of part.chapters) {
			if (chapter.entry !== chapterEntry) continue;
			return [...chapter.lessons].sort(compareLessons).flatMap((lesson) => {
				if ((!showDrafts && lesson.draft) || !isPublished(lesson.slug)) return [];
				const label = headings.get(lesson.entry)?.(lesson.title) ?? lesson.title;
				return [{ href: `/${lesson.slug}/`, label }];
			});
		}
	}
	return undefined;
}

/** 键是相对内容目录的路径，如 course/ownership/ch01/minimal-model.mdx。值把裸标题变成带编号的标题。 */
export function courseHeadings(contentDir: string): Map<string, (text: string) => string> {
	const locale = courseLocale(contentDir);
	const course = loadCourse(contentDir);
	const map = new Map<string, (text: string) => string>();
	course.parts.filter(part => !part.unnumbered).forEach((part, partIndex) => {
		if (part.entry) {
			const index = partIndex + 1;
			map.set(part.entry, (text) => partHeading(locale, index, text));
		}
		for (const chapter of part.chapters) {
			if (chapter.entry) {
				map.set(chapter.entry, (text) => chapterHeading(locale, chapter.num, text));
			}
			[...chapter.lessons].sort(compareLessons).forEach((lesson, index) => {
				map.set(lesson.entry, (text) => lessonHeading(chapter.num, index + 1, text));
			});
		}
	});
        for (const chapter of course.appendix?.chapters ?? []) {
                const letter = appendixLetter(chapter.num);
                if (chapter.entry) {
                        map.set(chapter.entry, (text) => appendixChapterHeading(locale, chapter.num, text));
                }
                [...chapter.lessons].sort(compareLessons).forEach((lesson, index) => {
                        map.set(lesson.entry, (text) => `${letter}.${index + 1} ${stripLesson(text)}`);
                });
        }
	return map;
}
