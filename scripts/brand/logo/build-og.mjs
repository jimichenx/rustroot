/**
 * 生成社交分享图 public/og/zh.png 与 public/og/en.png（1200×630）。
 *
 * 锁扣不另画一份：mark 与字标轮廓都从 src/components/starlight/SiteTitle.astro
 * 里取，所以顶栏改了分享图会跟着改。文案取自两站首页 hero（src/content/docs 下各自的
 * index.mdx），改了 hero 标题记得回来同步。
 *
 * 用法：node scripts/brand/logo/build-og.mjs [zh|en]（不给参数就两种都出）
 * 依赖：本机 Google Chrome（只用于截图，不进构建流程）。
 */
import { existsSync, readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { execFileSync } from 'node:child_process';

const CHROME = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const ROOT = resolve(import.meta.dirname, '../../..');

const siteTitle = readFileSync(join(ROOT, 'src/components/starlight/SiteTitle.astro'), 'utf8');
const mark = siteTitle.match(/<svg class="mark"[\s\S]*?<\/svg>/)[0].replace(/ class="mark"/, '');
const wordmark = siteTitle
	.match(/<svg class="wordmark"[\s\S]*?<\/svg>/)[0]
	.replace(/ class="wordmark"/, '');

const SITES = {
	zh: {
		domain: 'rustroot.cn',
		tagline: '跨越语言惯性，学会像 Rust 一样思考',
		sub: '不堆砌零散的语法词典，带你跨越过往语言的习惯，从内存与类型出发建立一套自洽的 Rust 心智模型。',
	},
	en: {
		domain: 'rustroot.dev',
		tagline: 'Understand the constraints before you learn Rust',
		sub: 'An advanced Rust course that derives the language from memory, calls, and data flow.',
	},
};

/** 放大并压暗的换轨方印，从右侧出血，与左上主 mark 几何同构 */
const rings = `
<svg class="rings" viewBox="0 0 64 64" fill="none" aria-hidden="true">
	<path d="M20 6H44C51.732 6 58 12.268 58 20V23H36.76L24.76 35H6V20C6 12.268 12.268 6 20 6Z" fill="#262932"/>
	<path d="M39.24 29H58V44C58 51.732 51.732 58 44 58H20C12.268 58 6 51.732 6 44V41H27.24L39.24 29Z" fill="#5c2a12"/>
</svg>`;

const page = (locale) => `<!doctype html>
<html lang="${locale === 'zh' ? 'zh-CN' : 'en'}"><head><meta charset="utf-8"><style>
	:root { color-scheme: dark; }
	* { box-sizing: border-box; margin: 0; }
	html, body { width: 1200px; height: 630px; overflow: hidden; }
	body {
		position: relative;
		background: #17181c;
		font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Helvetica Neue", Arial,
			"PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", sans-serif;
		-webkit-font-smoothing: antialiased;
	}
	.rings { position: absolute; right: -190px; top: 50%; transform: translateY(-50%); width: 700px; height: 700px; }
	.wrap { position: absolute; left: 96px; top: 0; width: 760px; height: 100%; }
	.lockup { position: absolute; top: 104px; display: flex; align-items: center; gap: 20px; color: #fff; }
	.lockup svg:first-child { width: 84px; height: 84px; }
	.lockup svg:last-child { height: 46px; width: auto; }
	h1 { position: absolute; top: 262px; font-size: 52px; line-height: 1.22; font-weight: 700;
		letter-spacing: .01em; color: #fff; max-width: 720px; }
	p { position: absolute; top: 400px; font-size: 23px; line-height: 1.6; color: #888b96; max-width: 620px; }
	.domain { position: absolute; left: 96px; bottom: 62px; font-family: ui-monospace, "SF Mono",
		Menlo, Consolas, monospace; font-size: 17px; letter-spacing: .06em; color: #c0c2c7; }
	.rule { position: absolute; left: 96px; bottom: 108px; width: 1008px; height: 1px; background: #353841; }
</style></head><body>
${rings}
<div class="wrap">
	<div class="lockup">${mark}${wordmark}</div>
	<h1>${SITES[locale].tagline}</h1>
	<p>${SITES[locale].sub}</p>
</div>
<div class="rule"></div>
<div class="domain">${SITES[locale].domain}</div>
</body></html>`;

const out = join(ROOT, 'public/og');
const only = process.argv[2];
const dir = mkdtempSync(join(tmpdir(), 'rustroot-og-'));
for (const locale of ['zh', 'en']) {
	if (only && only !== locale) continue;
	const html = join(dir, `${locale}.html`);
	writeFileSync(html, page(locale));
	const png = join(out, `${locale}.png`);
	try {
		execFileSync(CHROME, [
			'--headless=new',
			'--disable-gpu',
			'--hide-scrollbars',
			'--no-first-run',
			'--no-default-browser-check',
			'--disable-extensions',
			'--disable-background-networking',
			`--user-data-dir=${join(dir, `profile-${locale}`)}`,
			'--force-device-scale-factor=1',
			'--window-size=1200,630',
			`--screenshot=${png}`,
			`file://${html}`,
		], { stdio: 'ignore', timeout: 120_000 });
	} catch (err) {
		// headless Chrome 写完截图后偶尔不退出，超时被杀。文件在就算成功。
		if (!existsSync(png)) throw err;
	}
	console.log(png);
}
rmSync(dir, { recursive: true, force: true });
