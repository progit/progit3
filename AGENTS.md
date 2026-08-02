# AGENTS.md

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
  `progit.mobi` — `asciidoctor-epub3` 2.1.3 no longer produces KF8/MOBI. This is
  pre-existing repo behavior, not an environment problem.
- Generated outputs (`progit.html`, `progit.pdf`, `progit.epub`, `progit.fb2.zip`, etc.),
  `book/contributors.txt`, and `Gemfile.lock` are all git-ignored.

### Local reader (manual/GUI verification)
- `./builder/target/release/progit serve --port 8080` builds the HTML and serves a
  multi-page reader at `http://127.0.0.1:8080/` (binds `127.0.0.1` only). `/` is the cover +
  TOC; `/r/<section>` is a single section with sidebar TOC and prev/next links. Ctrl-C to stop.
