# `progit` — Pro Git build system

A small, self-contained Rust binary that builds the Pro Git book in every
supported format and can serve the HTML edition on a local web server for
offline reading.

It is a modern, single-command replacement for the `rake` tasks. Under the hood
it drives the same [Asciidoctor](https://asciidoctor.org) toolchain, so the
output is identical — it just gives you one tool, a nicer CLI, and a built-in
reader.

## Requirements

- A Rust toolchain (`cargo`) to build the binary — no third-party crates are
  used, so it compiles offline.
- The Asciidoctor toolchain for the actual conversions, installed via Bundler
  from the repository's `Gemfile`:

  ```sh
  bundle install
  ```

  The `serve`/`html` commands only need `asciidoctor`; `pdf`, `epub` and `mobi`
  need `asciidoctor-pdf` and `asciidoctor-epub3` respectively.

## Building the binary

```sh
cd builder
cargo build --release
# binary at builder/target/release/progit
```

Optionally install it onto your `PATH`:

```sh
cargo install --path builder
```

## Usage

Run from anywhere inside the repository (it finds `progit.asc` by walking up
from the current directory).

```sh
progit html            # single-file progit.html (images/CSS embedded)
progit pdf             # progit.pdf
progit epub            # progit.epub
progit mobi            # progit.mobi (KF8)
progit all             # build every format
progit serve           # build HTML and serve it at http://127.0.0.1:8080/
progit contributors    # regenerate book/contributors.txt if stale
progit clean           # remove all generated files
```

### Reading the book locally

```sh
progit serve --open            # build, serve, and open your browser
progit serve --port 3000       # choose a port
progit serve --no-build        # serve an already-built progit.html
```

The server is a minimal, read-only static file server bound to `127.0.0.1`.
`/` redirects to the book; paths are sandboxed to the repository directory.
Press `Ctrl-C` to stop.

### Options

| Option          | Meaning                                                        |
| --------------- | -------------------------------------------------------------- |
| `--no-bundle`   | Call the `asciidoctor*` executables directly, not `bundle exec`. |
| `-p, --port N`  | Port for `serve` (default `8080`).                             |
| `--open`        | Open the book in a browser once `serve` is ready.             |
| `--no-build`    | For `serve`: skip rebuilding and serve the existing HTML.     |
| `-V, --version` | Print the tool version.                                        |
| `-h, --help`    | Show usage.                                                    |

## How it maps to the old Rakefile

- Version number: latest git tag `x.y.z` with the patch bumped (or `0` when the
  repo has no tags), passed as `revnumber`.
- Build date: today, passed as `revdate`.
- `book/contributors.txt` is regenerated from `git shortlog` whenever its
  recorded commit no longer matches `HEAD`.

By default the tool runs the Asciidoctor gems through `bundle exec` when a
`Gemfile` is present; pass `--no-bundle` to use globally installed gems.
