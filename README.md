# Rust Root

Rust Root is a course about the constraints behind Rust, not a syntax tour. It starts from what a program has to do at runtime, then derives the language's choices: ownership, borrowing, types, and the rules that follow from them. The official Rust Book remains the place to learn syntax and do exercises. This course explains why those forms exist.

Chinese is the language lessons are written in. English pages use the same paths and are translated after a Chinese part is stable. The site is not an official Rust project and is not endorsed by the Rust Foundation.

The repository is the source of the course site. Lessons are published as they are written.

| Site | Language | Status |
|---|---|---|
| https://rustroot.cn | Simplified Chinese | Published. |
| https://rustroot.dev | English | Local only. Not deployed yet. |

## Stack

- [Astro](https://astro.build) 7 and [Starlight](https://starlight.astro.build) 0.42
- Static HTML. One codebase, two builds, two domains. URLs have no locale prefix.
- Course programs live in `examples/`, a Cargo workspace, so the code on the page is the code that builds.

## Develop

```bash
npm install

npm run dev        # Chinese site, http://localhost:4321
npm run dev:en     # English site, http://localhost:4322

npm run build      # both sites → dist/zh and dist/en
npm run build:zh   # Chinese only
npm run build:en   # English only
npm run preview    # preview dist/zh
npm run preview:en # preview dist/en
```

The two dev servers can run at the same time. The language switcher jumps to the other port and the same path. `npm run dev` holds Astro's dev lock. `npm run dev:en` does not, so stop that process in its own terminal.

If install or dev fails with `EPERM ... Library/Preferences/astro`, set `ASTRO_TELEMETRY_DISABLED=1`.

## Layout

```
rustroot/
├── astro.config.ts                 Astro and Starlight config
├── src/
│   ├── config/site.ts              The two domains and languages
│   ├── config/course-slugs.ts      Public URLs for parts, chapters, and lessons
│   ├── config/sidebar.ts           Sidebar built from course/
│   ├── content.config.ts           Loads only the current language
│   ├── content/docs/
│   │   ├── zh-cn/                  Chinese pages (written first)
│   │   └── en/                     English pages (same relative paths)
│   ├── content/i18n/               Starlight UI strings
│   ├── components/starlight/       Header, sidebar, footer, hreflang
│   ├── styles/                     Site CSS and diagram colors
│   └── plugins/                    Code block plugins (Playground link, collapse)
├── examples/                       Course programs (Cargo workspace)
├── public/                         Favicon, social images
└── scripts/                        Dev setup and the logo artwork
```

## URLs

A part, a chapter, and a lesson do not use the `chNN` folder name in the public URL.

| Page | File | URL |
|---|---|---|
| Catalog | `course/index.md` | `/course/` |
| Part | `course/ownership/index.md` | `/ownership/` |
| Chapter | `course/ownership/ch01/index.md` with `id: rules` | `/ownership/rules/` |
| Lesson | `course/ownership/ch01/hard-triangle.mdx` | `/ownership/hard-triangle/` |

`rustroot.cn/<path>` and `rustroot.dev/<path>` are hreflang alternates.


## License

- **Text, figures, and course pages** are licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.en). You may share them for non-commercial use if you credit the author and link to [rustroot.dev](https://rustroot.dev). Commercial use, paid training, and adaptations need written permission.
- **Example code** under `examples/` is licensed under the [MIT License](LICENSE-MIT).

Copyright © 2026 RustRoot.
