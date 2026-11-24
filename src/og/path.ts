import { createHash } from 'node:crypto';

export interface OgCardText {
	slug: string;
	title: string;
	share: string;
}

/** 句子或标题一改，文件名里的这 8 位就变。微信按图片 URL 缓存分享卡。 */
export function ogHash(title: string, share: string): string {
	return createHash('sha256').update(`${title}\n${share}`).digest('hex').slice(0, 8);
}

/** 站点根相对路径，以 / 开头。slug 是公开地址，如 memory/hard-triangle。 */
export function ogPublicPath(card: OgCardText): string {
	return `/og/${card.slug}-${ogHash(card.title, card.share)}.png`;
}
