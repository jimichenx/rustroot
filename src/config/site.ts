/**
 * 站点级配置：双域名 / 双语言的唯一事实来源。
 *
 * - rustroot.cn  → 简体中文（国内站）
 * - rustroot.dev → English（国际站）
 *
 * 同一份代码通过环境变量 `SITE_LOCALE=zh|en` 分别构建两次，
 * 每次构建只包含对应语言的内容，URL 不带语言前缀。
 * 语言切换与 hreflang 通过 `otherLocale()` 指向对方域名的同一路径。
 */

export type SiteLocale = 'zh' | 'en';

export interface LocaleConfig {
	/** 构建标识，同时也是 `SITE_LOCALE` 的取值 */
	code: SiteLocale;
	/** BCP-47 语言标签，写入 `<html lang>` 与 hreflang */
	lang: string;
	/** 语言切换器中显示的名称 */
	label: string;
	/** 生产环境站点地址（不带尾部斜杠） */
	site: string;
	/** 本地开发端口，便于两种语言的 dev server 同时运行 */
	devPort: number;
	/** 内容目录：`src/content/docs/<contentDir>/` */
	contentDir: string;
	/** 站点标题与描述 */
	title: string;
	description: string;
}

export const LOCALES: Record<SiteLocale, LocaleConfig> = {
	zh: {
		code: 'zh',
		lang: 'zh-CN',
		label: '简体中文',
		site: 'https://rustroot.cn',
		devPort: 4321,
		contentDir: 'zh-cn',
		title: 'Rust Root',
		description:
			'从根上理解 Rust 的开源指南：从内存与数据流出发，重新推导 Rust 的设计选择，建立属于 Rust 的思维模型。',
	},
	en: {
		code: 'en',
		lang: 'en',
		label: 'English',
		site: 'https://rustroot.dev',
		devPort: 4322,
		contentDir: 'en',
		title: 'Rust Root',
		description:
			"An open guide to understanding Rust from the ground up: start from memory and data flow, re-derive Rust's design decisions, and build a mental model that is truly Rust's own.",
	},
};

function resolveLocale(): SiteLocale {
	const raw = (process.env.SITE_LOCALE ?? 'zh').toLowerCase();
	if (raw === 'zh' || raw === 'en') return raw;
	throw new Error(`Invalid SITE_LOCALE "${raw}". Expected "zh" or "en".`);
}

/** 当前正在构建 / 开发的语言 */
export const SITE_LOCALE: SiteLocale = resolveLocale();

/** 当前语言的完整配置 */
export const CURRENT: LocaleConfig = LOCALES[SITE_LOCALE];

/** 对方语言（用于语言切换与 hreflang） */
export const OTHER: LocaleConfig = LOCALES[SITE_LOCALE === 'zh' ? 'en' : 'zh'];

/**
 * 某个语言站点的基础 URL。
 * 开发模式下指向对应语言 dev server 的 localhost 端口，
 * 便于本地同时运行 `npm run dev` 与 `npm run dev:en` 并互相切换。
 */
export function siteOrigin(locale: LocaleConfig, isDev: boolean): string {
	return isDev ? `http://localhost:${locale.devPort}` : locale.site;
}

/** hreflang 的 x-default 指向国际站（英文） */
export const X_DEFAULT_LOCALE: LocaleConfig = LOCALES.en;
