import assert from 'node:assert/strict';
import test from 'node:test';
import { createServer } from 'vite';

const server = await createServer({
        configFile: false,
        logLevel: 'silent',
        server: { middlewareMode: true },
});
const { appendixChapterHeading, courseHeadings } = await server.ssrLoadModule('/src/config/course-numbers.ts');
await server.close();

test('appendix headings remove repeated cached prefixes', () => {
        assert.equal(appendixChapterHeading('zh', 2, '机制详解'), '附录 B　机制详解');
        assert.equal(appendixChapterHeading('zh', 2, '附录 B　附录 B　机制详解'), '附录 B　机制详解');
        assert.equal(appendixChapterHeading('en', 2, 'Appendix B　Appendix B　Mechanisms'), 'Appendix B　Mechanisms');
});

test('appendix headings remain stable across repeated loads and renumbering', () => {
        const heading = courseHeadings('zh-cn').get('appendix/mechanisms/index.md');
        assert.ok(heading);
        const once = heading('机制详解');
        assert.equal(heading(heading(once)), once);
        assert.equal(appendixChapterHeading('zh', 3, once), '附录 C　机制详解');
        assert.equal(appendixChapterHeading('en', 3, 'Appendix B　Mechanisms'), 'Appendix C　Mechanisms');
});
