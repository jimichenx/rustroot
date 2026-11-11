import type { APIRoute } from 'astro';

/**
 * robots.txt 按域名动态生成：sitemap 地址随 `site` 配置（rustroot.cn / rustroot.dev）变化。
 * Starlight 在设置了 `site` 时会自动生成 sitemap-index.xml。
 */
export const GET: APIRoute = ({ site }) => {
	if (site?.hostname === 'preview.rustroot.cn') {
		const body = ['User-agent: *', 'Disallow: /', ''].join('\n');
		return new Response(body, {
			headers: { 'Content-Type': 'text/plain; charset=utf-8' },
		});
	}
	const sitemapURL = new URL('sitemap-index.xml', site);
	const body = ['User-agent: *', 'Allow: /', '', `Sitemap: ${sitemapURL.href}`, ''].join('\n');
	return new Response(body, {
		headers: { 'Content-Type': 'text/plain; charset=utf-8' },
	});
};
