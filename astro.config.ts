import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { satteri } from '@astrojs/markdown-satteri';
import { CURRENT, REPOSITORY, SITE_LOCALE } from './src/config/site';
import { isPublished } from './src/config/release';
import { sidebar } from './src/config/sidebar';
import { chapterLessonsPlugin } from './src/plugins/chapter-lessons';
import { exampleSourcePlugin } from './src/plugins/example-source';
import { cjkEmphasisPlugin } from './src/plugins/cjk-emphasis';
import { unpublishedLinkPlugin } from './src/plugins/unpublished-links';
import { wrapTablesPlugin } from './src/plugins/wrap-tables';

function releasedRedirects(map: Record<string, string>): Record<string, string> {
	return Object.fromEntries(
		Object.entries(map).filter(([, target]) => isPublished(target.replace(/^\/+|\/+$/g, ''))),
	);
}

export default defineConfig({
	// 每次构建只面向一个域名；canonical、sitemap、og:url 均基于此。
	site: CURRENT.site,
        // Keep published URLs working after the first part's rename.
        redirects: releasedRedirects({
                '/ownership/hard-triangle/': '/memory/hard-triangle/',
                ...(SITE_LOCALE === 'zh'
                        ? {
                                '/ownership/': '/memory/',
                                '/ownership/rules/': '/memory/ownership/',
                                '/ownership/stack-and-heap/': '/memory/stack-and-heap/',
                                '/ownership/minimal-model/': '/memory/minimal-model/',
                                '/ownership/references/': '/memory/references/',
                                '/ownership/lifetimes/': '/memory/lifetimes/',
                                '/ownership/summary/': '/memory/summary/',
                        }
                        : {}),
        }),
	markdown: {
		processor: satteri({
			mdastPlugins: [cjkEmphasisPlugin, unpublishedLinkPlugin],
			hastPlugins: [wrapTablesPlugin, exampleSourcePlugin, chapterLessonsPlugin],
		}),
	},
	server: { port: CURRENT.devPort },
	integrations: [
		starlight({
			title: CURRENT.title,
			description: CURRENT.description,
			// 单一 root locale：内容直接挂在站点根路径，无 /zh-cn/ 或 /en/ 前缀。
			// 另一种语言由另一次构建、另一个域名提供，见 src/config/site.ts。
			locales: {
				root: { label: CURRENT.label, lang: CURRENT.lang },
			},
			customCss: ['./src/styles/custom.css', './src/styles/diagrams.css'],
			components: {
				// hreflang / og:locale 等跨域 SEO 元信息
				Head: './src/components/starlight/Head.astro',
				// 顶栏字标：Instrument Sans 的 Rust Root，两个 R 为锈橙
				SiteTitle: './src/components/starlight/SiteTitle.astro',
				// 语言切换器：跳转到对方域名的同一路径
				LanguageSelect: './src/components/starlight/LanguageSelect.astro',
				// 侧栏：vendored SidebarSublist——章组标题即章描述页链接
				Sidebar: './src/components/starlight/Sidebar.astro',
				// 右侧大纲：标题行可收起，收起后右栏缩成窄轨
				TableOfContents: './src/components/starlight/TableOfContents.astro',
				// 页脚：CC BY-NC-ND 4.0；备案号只在中文站
				Footer: './src/components/starlight/Footer.astro',
			},
			sidebar,
			lastUpdated: true,
			credits: false,
			social: [{ icon: 'github', label: 'GitHub', href: REPOSITORY }],
			editLink: { baseUrl: `${REPOSITORY}/edit/develop/` },
		}),
	],
	// 两台 dev server 不能共用 Vite 预构建缓存，否则后启动的语言会盖掉先启动的模块。
	cacheDir: `./node_modules/.astro-cache-${SITE_LOCALE}`,
	vite: {
		cacheDir: `node_modules/.vite-${SITE_LOCALE}`,
		define: {
			// 让客户端脚本也能知道当前语言（目前仅调试用途）
			__SITE_LOCALE__: JSON.stringify(SITE_LOCALE),
		},
		build: {
			rolldownOptions: {
				onLog(level, log, defaultHandler) {
					// Astro 在每个 MDX 里注入 "use astro:head-inject"。Rolldown 不认识这条指令，
					// 会按文件各打一条警告，构建结果不受影响。
					if (
						log.code === 'MODULE_LEVEL_DIRECTIVE' &&
						String(log.message).includes('astro:head-inject')
					) {
						return;
					}
					defaultHandler(level, log);
				},
			},
		},
	},
});
