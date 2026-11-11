/**
 * Astro writes the content layer to `<root>/.astro/`. Two dev servers in one
 * project share that directory, so the later sync overwrites the earlier
 * language and the English site starts serving Chinese pages.
 *
 * Point each process at `.astro-zh/` or `.astro-en/` by patching the one
 * hardcoded path. A formal build that sets RELEASE_POINTER_FILE uses
 * `.astro-zh-release/` instead, so its shorter module map does not replace
 * the one the dev server is serving. Re-run after `npm install`, before
 * `dev` / `build`.
 */
import fs from 'node:fs';

const file = new URL('../node_modules/astro/dist/core/config/settings.js', import.meta.url);
const source = fs.readFileSync(file, 'utf8');
const original = 'const dotAstroDir = new URL(".astro/", config.root);';
const localeOnly =
	'const dotAstroDir = new URL(`.astro-${process.env.SITE_LOCALE === "en" ? "en" : "zh"}/`, config.root);';
const patched =
	'const dotAstroDir = new URL(`.astro-${process.env.SITE_LOCALE === "en" ? "en" : "zh"}${process.env.RELEASE_POINTER_FILE ? "-release" : ""}/`, config.root);';

if (source.includes(patched)) {
	process.exit(0);
}

const current = [localeOnly, original].find((line) => source.includes(line));
if (!current) {
	console.error(
		'ensure-locale-astro-dir: Astro settings.js no longer contains the expected .astro path. Update the patch.',
	);
	process.exit(1);
}

fs.writeFileSync(file, source.replace(current, patched));
