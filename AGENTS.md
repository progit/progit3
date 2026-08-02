# Guidance for AI Agents

This repository is the source for _Pro Git_ (3rd Edition). Automated agents may help
with this book, but under one firm rule.

## The prose rule: humans write the book

**Agents must never write prose for the book.** Every sentence a reader reads — the
actual explanatory text of the chapters, sections, sidebars, and captions — is written
by a human author. This is a book with named authors and a voice; the writing is the
work, and it is not delegated to a machine.

This is not a style preference to be weighed against convenience. If a task would have an
agent compose, rewrite, paraphrase, expand, or "polish" the book's sentences, the agent
must stop and hand it back to a human, even when the change seems small or obviously
helpful.

## What agents *may* do

Agents are welcome to take on the mechanical and supporting work around the prose:

- **Minor search-and-replace** — e.g. renaming `master` → `main` in examples, fixing a
  command flag, correcting a typo or a broken link. Mechanical substitutions, not rewrites.
- **Add or update images and figures** — generate, place, and wire up diagrams and
  screenshots (following the figure process in `CONTRIBUTING.md`).
- **Help plan** — build revision plans, change inventories, checklists, and scope analyses
  (as in `REVISION_PLAN.md`).
- **Research** — investigate Git behavior, releases, version history, and command changes;
  report findings for a human to write up.
- **Rearrange content** — move existing sections, reorder material, split or merge files,
  fix cross-references and includes — as long as the sentences themselves are not rewritten.

## The line

The test is simple: **does the change put new or altered sentences in front of the
reader?** If yes, a human writes it. If the agent is moving, replacing, illustrating,
researching, or planning around prose that a human wrote, that's fair game.

When in doubt, treat it as prose and hand it to a human.

## Blog posts are the exception — but read the style guide first

The site's blog (`site/src/content/blog/`) is not the book. Agents may write and edit
blog posts when asked — automated posts exist precisely to communicate project updates.
Two hard requirements:

1. **Before writing or editing any blog post, read `site/BLOG_STYLE.md` and follow
 it.** The blog is written in the book's voice, and that guide defines it. Do not
 write a post without loading it first.
2. **Set `automated: true` in the frontmatter of every agent-written post.** That
 renders the "written by AI" disclosure banner; unlabeled machine writing is never
 published.

## Cursor Cloud specific instructions

This repo is the **Pro Git** book (source in AsciiDoc). "Building" means converting
`progit.asc` into HTML/EPUB/FB2/MOBI/PDF. There is no runtime backend or database.

### Toolchains
- **Ruby/Rake pipeline** (this is what CI runs). Ruby 3.2 is installed via apt; Bundler
  is installed system-wide and gems are installed into `~/.bundle-gems` (configured via a
  global `bundle config path`). The startup update script runs `bundle install` to refresh
  gems, so you do not need to reinstall them.
- **Rust `progit` CLI** in `builder/` — an optional single-command wrapper around the same
  Asciidoctor toolchain, plus a local book reader. It has no third-party crates. Build with
  `cargo build --release --manifest-path builder/Cargo.toml` (binary at
  `builder/target/release/progit`). See `builder/README.md`.

### Build / test / lint (standard commands live in `Rakefile` and `README.asc`)
- Full build (CI parity): `bundle exec rake book:build`. This also runs the
  htmlproofer + epubcheck validation step, but **`book:build` intentionally rescues and
  ignores those check errors** (see the `rescue` in `Rakefile`).
- Single formats: `bundle exec rake book:build_html` / `build_epub` / `build_fb2` /
  `build_mobi` / `build_pdf`. The PDF build is the slow one (~1 min).
- There is no separate lint step; htmlproofer/epubcheck (run via `book:check`) are the de
  facto linters for the generated HTML/EPUB.

### Non-obvious gotchas
- **Do not use `bundle exec rake book:ci` for offline verification.** Unlike `book:build`,
  `book:ci` does *not* ignore htmlproofer/epubcheck errors, and htmlproofer will fail on
  external links (404s / no network) even when the book itself is fine. Use `book:build`.
- **MOBI is effectively a no-op.** `book:build_mobi` reports success but does not emit
  `progit.mobi` — current `asciidoctor-epub3` (see `Gemfile`) no longer produces KF8/MOBI.
  This is pre-existing repo behavior, not an environment problem.
- Generated outputs (`progit.html`, `progit.pdf`, `progit.epub`, `progit.fb2.zip`, etc.),
  `book/contributors.txt`, and `Gemfile.lock` are all git-ignored.

### Local reader (manual/GUI verification)
- `./builder/target/release/progit serve --port 8080` builds the HTML and serves a
  multi-page reader at `http://127.0.0.1:8080/` (binds `127.0.0.1` only). `/` is the cover +
  TOC; `/r/<section>` is a single section with sidebar TOC and prev/next links. Ctrl-C to stop.
