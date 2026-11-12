/** 消息在管子里的可放区域。两端圆角各让出半个管高，避免块压到轮廓上。 */
export function pipeMetrics(x: number, y: number, width: number, height: number) {
	const cap = height / 2;
	const pad = 4;
	return {
		midY: y + height / 2,
		boreX: x + cap + pad,
		boreY: y + pad,
		boreW: Math.max(0, width - (cap + pad) * 2),
		boreH: Math.max(0, height - pad * 2),
	};
}
