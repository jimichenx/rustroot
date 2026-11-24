import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { collectOgCards } from './collect';
import { ogPublicPath } from './path';
import { renderOgPng } from './render';

const out = process.argv[2] ?? path.join(process.cwd(), '.og-preview');
const cards = collectOgCards();
for (const card of cards) {
	const rel = ogPublicPath(card).replace(/^\//, '');
	const file = path.join(out, rel);
	await mkdir(path.dirname(file), { recursive: true });
	await writeFile(file, await renderOgPng(card));
	console.log(`${card.title}  ${rel}`);
}
console.log(`共 ${cards.length} 张`);
