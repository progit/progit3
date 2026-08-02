# Pro Git, 3rd Edition — Revision Plan

Planning document for the third edition of _Pro Git_, timed to coincide with the
**Git 3.0** release. It inventories what is out of date in the current (2nd-edition)
text on two axes:

- **🕰 General staleness** — wrong or dated regardless of Git 3.0.
- **⚡ 3.0-specific** — driven by the Git 3.0 breaking changes.

## Git 3.0 status (as of this writing)

There is **no firm release date**. Maintainers have discussed targeting roughly the
**end of 2026**, conditional on SHA-256 interoperability maturing and major forges
being ready. The concrete, scheduled signal is the Rust rollout ramp:

| Version | Rust status |
|---------|-------------|
| 2.52    | Rust auto-detected |
| 2.55    | Rust enabled by default |
| **3.0** | Rust **mandatory** |

A designated **LTS release** (the last 2.x before 3.0) will get bug fixes for 4 cycles
and security fixes for 6 — that tag is the clearest "3.0 is imminent" marker to watch.

**The Git 3.0 breaking changes that drive this edition:**

1. **SHA-256** becomes the default object hash for new repositories (SHA-1 still supported; interop between the two).
2. **`reftable`** replaces the `files` backend as the default ref storage.
3. **`main`** becomes the actual default branch name.
4. **`safe.bareRepository`** default flips from `all` → `explicit`.
5. **Rust** becomes a mandatory build dependency.
6. **Removals**: grafts, `git pack-redundant`, `git whatchanged`, legacy `branch`/`remote` dirs.

Sources: [Git BreakingChanges doc](https://git-scm.com/docs/BreakingChanges),
[Phoronix: Git 3.0 release talk](https://www.phoronix.com/news/Git-3.0-Release-Talk-2026).

---

## Part 0 — Cross-cutting changes (touch nearly every chapter)

Do these as coordinated book-wide passes, not per-chapter, to avoid inconsistency.

| # | Change | Scope | Notes |
|---|--------|-------|-------|
| **X1** | **`master` → `main` default** | **602 renames across the book** (615 total `master` hits) | The book teaches master-first and treats `main` as an opt-in override (`first-time-setup.asc:89`). In 3.0, `main` is _the_ default. Full per-line breakdown in [`book_master_to_main_inventory.md`](book_master_to_main_inventory.md). Largest single edit in the book; also requires regenerating diagrams and screenshots that show the branch. |
| **X2** | **SHA-1 → SHA-256 default hash** | ~31 files reference hashes | `what-is-git.asc:56` states "The mechanism that Git uses… is called a SHA-1 hash… a 40-character string." Under 3.0 new repos are SHA-256 (64 hex chars). Reframe to "a cryptographic hash (SHA-256 by default; SHA-1 for older repos)" and add the interop story. Decide a single policy for example hashes: regenerate at 64 chars, or keep labeled SHA-1 legacy examples. |
| **X3** | **Version framing** | Whole book | `installing.asc:9` says "written using Git version 2." Bump to 3.x. The many "since Git 2.23 / 2.27 / 2.28…" notes (restore/switch, `pull.rebase` warning, `init.defaultBranch`) now describe ancient history — reframe as baseline behavior, not new features. |
| **X4** | **`safe.bareRepository` `all`→`explicit`** | Not currently covered | New security default. Add to Ch 8 (config); caveat in Ch 7/Ch 10. `safe.directory` (2022 CVE fix) is also absent from the book entirely. |
| **X5** | **Screenshots & UI refresh** | Ch 4, Ch 6, Appendix A | All forge/IDE screenshots are years stale. Flagged per chapter below. |

---

## Chapter-by-chapter

### Ch 1 — Getting Started
- 🕰 `what-is-git.asc`: three-states model, history, philosophy hold up — most durable chapter. History section can note Git's maturation and the 3.0 transition.
- 🕰 `installing.asc`: version note (X3); refresh per-platform install steps. **Add the Rust toolchain requirement** — 3.0 makes Rust mandatory, so "compile from source" must cover installing Rust/cargo.
- 🕰 `first-time-setup.asc:86-96`: rewrite the default-branch subsection — flips from "how to change it" to "it's `main`; how to override if needed."
- ⚡ `what-is-git.asc:51-63`: the headline SHA-1 passage (X2).

### Ch 2 — Git Basics
- 🕰 `undoing.asc:153-155`: `restore`/`switch` framed as "new in 2.23" — normalize as standard; consider making them the primary taught commands over `checkout`/`reset`.
- 🕰 `remotes.asc:132`: `pull.rebase` warning framed as "since 2.27."
- ⚡ Example hashes throughout (`recording-changes` 35 refs, `tagging`, `viewing-history`) are SHA-1 (X2).
- ⚡ `getting-a-repository.asc`: `git init` now yields `main` and (3.0) a reftable backend — add a forward-reference note.

### Ch 3 — Git Branching
- ⚡ **Heaviest `master` concentration** (101 renames): `basic-branching-and-merging` (22), `rebasing` (24), `remote-branches` (18), `branch-management` (18), `nutshell` (15). All canonical branch diagrams say `master` — `diagram-source/` needs regenerating.
- 🕰 Content is solid; mostly the X1 sweep + diagram regeneration.
- ⚡ `branch-management.asc:82,131` already discuss `master/main/mainline` renaming — keep the concept, revisit wording for a main-default world.

### Ch 4 — Git on the Server
- 🕰 **Most dated infrastructure chapter.** `git-daemon`, `gitweb`, and hand-rolled `setting-up-server` describe near-unused practices. Demote Gitweb/daemon; lead with modern self-hosting. **Gitea/Forgejo is not mentioned at all** and should be added.
- 🕰 `protocols`/`smart-http`: dumb HTTP is effectively dead; **protocol v2** (default since 2.26) needs proper coverage.
- 🕰 `generating-ssh-key.asc`: recommend **Ed25519** as default.
- ⚡ Sidebar on reftable + SHA-256 hosting/interop implications (forge readiness is what gates 3.0's date).

### Ch 5 — Distributed Git
- 🕰 X1 sweep: `contributing` (48 renames), `maintaining` (39). Content (contributing workflows, `format-patch`/`am`, integration-manager model) is durable.
- 🕰 Contextualize email-based workflow against PR-based norms; still valid for kernel/Git communities.

### Ch 6 — GitHub
- 🕰 **Fastest-rotting chapter.** All screenshots stale; PR review UI, org settings, account setup flows all changed. Full re-capture + text pass.
- 🕰 Missing modern surface area: **Actions, Codespaces, current PR review experience.** Scope decision — chapter is deliberately "GitHub as an example forge," not exhaustive.
- ⚡ Default-branch language in examples (26 renames in `2-contributing`).

### Ch 7 — Git Tools
- ⚡ `signing.asc`: **GPG-only** today. Add **SSH commit/tag signing** (`gpg.format=ssh`, since 2.34) — now the mainstream choice — and `gpgsm` (X.509). Significant content addition.
- ⚡ `replace.asc`: contains the book's only **grafts** discussion (17 `master` refs too) — grafts are **removed in 3.0**. Rework around `replace`/`commit-graph`; mark grafts removed.
- ⚡ `rewriting-history.asc`: add the new **`git history`** command (experimental, introduced Git 2.54 / April 2026; `fixup` added 2.55). It rewrites history by modifying specific commits and **automatically rebases descendant branches** — a much simpler mental model than interactive rebase. Cover its four subcommands: **`reword`** (change a commit message in place), **`split`** (interactively carve one commit into two by hunk), **`fixup`** (fold staged changes into an older commit via three-way merge), **`drop`** (remove a commit, replaying descendants onto its parent). Note the current limitations: experimental/behavior-may-change, no merge commits, no operations that would produce conflicts, cannot drop root/merge commits. Position it alongside interactive rebase as the recommended everyday tool for the common cases.
- 🕰 `credentials.asc`: **Git Credential Manager (GCM / `manager`)** is now the cross-platform standard — update legacy `wincred`/naming. macOS `osxkeychain` still fine.
- 🕰 `rewriting-history.asc`: `filter-branch` is deprecated and warns on use; lead with **`git filter-repo`** (note BFG). `reset`/`revision-selection`/`debugging (bisect)` durable.
- 🕰 Highest total `master` count of any chapter (147 renames): `submodules` (35), `revision-selection` (20), `advanced-merging` (17), `bundling` (14), `stashing-cleaning` (14), `subtree-merges` (13).

### Ch 8 — Customizing Git
- ⚡ `config.asc`: add **`safe.bareRepository`** and **`safe.directory`** (X4); `init.defaultBranch` as default-is-main; note SHA-256 (`--object-format`) and reftable (`extensions.refStorage`).
- 🕰 `hooks`, `attributes`, `policy` durable. `policy.asc` uses `master` (8×) in its enforced-workflow example.

### Ch 9 — Git and Other Systems
- 🕰 **Strongest candidate for deep cuts.** `git svn` retains users (trim). The **Mercurial** bridge (`client-hg`/`import-hg`, 23+ `master` refs) is largely unmaintained and Bitbucket dropped Hg hosting in 2020; **Perforce** (`git-p4`, 37 refs) is niche. Recommend: keep trimmed `git svn`, demote Hg/P4 to a short "bridges exist" section, lean on generic `import-custom.asc` fast-import.
- 🕰 Verify `git-p4`/`hg` tooling runs on modern Python (scripts predate the Python-2 sunset).

### Ch 10 — Git Internals
- ⚡ **The chapter most reshaped by 3.0** (70 renames).
  - `objects`/`packfiles`: object model taught as SHA-1 20-byte / 40-hex (X2) — needs a SHA-256 rewrite with an "object format" framing and the interop story.
  - `refs.asc:7-23`: teaches refs purely as loose files under `.git/refs` + packed-refs. 3.0 makes **reftable** the default — substantial addition needed (and the _why_: Windows/macOS case-collision + performance).
  - `transfer-protocols.asc`: fold in **protocol v2**.
- 🕰 `maintenance.asc`: `git maintenance` relatively current; verify against latest.

### Appendix A — Git in Other Environments
- 🕰 Editor/IDE coverage rots fast: `sublimetext`, `visualstudio`, `visualstudiocode`, `jetbrainsides` need version/screenshot refresh. `visualstudiocode.asc:5` still says "Git 2.0.0 or newer." Shell completion (`bash`/`zsh`/`powershell`) stable — version-check.

### Appendix B — Embedding Git
- 🕰 Binding versions drift: `libgit2`, `jgit`, `go-git`, `dulwich`. **Promote `go-git`** (now a mainstream pure-Go implementation). ⚡ Check each binding's **SHA-256 support status** — a real reader concern during the transition; add a per-library compatibility note.

---

## Command reference — new & deprecated commands

Commands that arrived **after the 2nd edition** (published Nov 2014, ~Git 2.1) or are
**deprecated/removed** by 3.0. Both lists drive edits to the running text _and_ to the
command index in `C-git-commands.asc`. Version numbers are the release that introduced or
changed each command; "experimental" means the man page still carries a
behavior-may-change warning.

### New commands (add coverage)

| Command | Since | Status | Where it belongs |
|---------|-------|--------|------------------|
| `git worktree` | 2.5 (2015) | stable | Ch 7 — multiple working trees from one repo |
| `git commit-graph` | 2.18 (2018) | stable | Ch 10 internals + performance story |
| `git range-diff` | 2.19 (2018) | stable | Ch 5 / Ch 7 — compare two versions of a patch series |
| `git multi-pack-index` (`git midx`) | 2.20 (2018) | stable | Ch 10 packfiles |
| `git switch` | 2.23 (2019) | stable (was experimental) | Ch 3 — the modern branch-switching verb; teach before `checkout` |
| `git restore` | 2.23 (2019) | stable (was experimental) | Ch 2 — the modern file-restore verb; teach before `checkout`/`reset` |
| `git sparse-checkout` | 2.25 (2020) | stable | Ch 7 + new monorepo-scale material |
| `git bugreport` | 2.27 (2020) | stable | Ch 7 debugging / Appendix |
| `git maintenance` | 2.30 (2020) | stable | Ch 10 maintenance + performance story |
| `git for-each-repo` | 2.31 (2021) | stable | Ch 8 / scripting |
| `scalar` | 2.38 (2022) | stable | New monorepo-scale material (bundled tool) |
| `git diagnose` | 2.39 (2022) | stable | Ch 7 debugging / Appendix |
| `git replay` | 2.44 (2024) | **experimental** | Ch 7 — server-side/bare history replay (no worktree touched) |
| `git backfill` | 2.49 (2025) | **experimental** | New partial-clone material — batch-download missing blobs |
| `git history` | 2.54 (2026); `fixup` 2.55 | **experimental** | Ch 7 — see rewriting-history entry above |

### Deprecated / removed by 3.0 (rewrite or excise)

| Command / feature | Status in 3.0 | Replacement | Book action |
|-------------------|---------------|-------------|-------------|
| `git whatchanged` | Removal planned; already needs `--i-still-use-this` | `git log` (with `--raw` for the old output) | Remove any use; note the retirement |
| `git pack-redundant` | Removal planned ("unusably slow"); needs `--i-still-use-this` | `git repack` / `git gc` | Not currently taught — leave out; mention in the removals note |
| **grafts** (`.git/info/grafts`) | **Removed** | `git replace` (incl. `--graft`) | Ch 7 `replace.asc` — rework, mark grafts removed (see Ch 7 above) |
| Legacy `$GIT_COMMON_DIR/branches/` & `/remotes/` | **Removed** | config-based remotes (`git remote` / `remote.*`) | Ch 10 refs / Ch 2 remotes — verify no examples rely on them |
| `git name-rev --stdin` | Option removed | `git name-rev --annotate-stdin` | Ch 7/10 — check for the old flag |
| `git filter-branch` | Deprecated (emits warning on use) | `git filter-repo` (external), `git replay` | Ch 7 `rewriting-history.asc` — lead with `filter-repo` (see Ch 7 above) |
| `git checkout` (overloaded modes) | Supported, but soft-superseded | `git switch` + `git restore` | Ch 2/3 — teach switch/restore first; keep `checkout` as the legacy all-in-one |

> Also removed as config (not commands, but adjacent): `core.commentString=auto` and
> `core.preferSymlinkRefs=true`. Note in Ch 8 config.
>
> Not scheduled for removal (still supported, don't cut): `git svn`, `git cvsimport`/
> `git cvsserver`/`git cvsexportcommit`, `git request-pull`, `git format-patch`/`git am`.
> The CVS/SVN bridges are staleness-driven trims (Ch 9), not 3.0 removals.

---

## Recommended new material for the 3rd edition

1. **A dedicated "Git 3.0 / migrating to SHA-256" section** — the marquee topic; consolidate the transition (interop repos, `--object-format`, forge readiness) rather than scattering it.
2. **Reftable** explainer (pairs with Ch 10 refs).
3. **SSH signing** (pairs with Ch 7).
4. **Security defaults** (`safe.*`) — didn't exist when the 2nd edition was written.
5. **Sparse-checkout / partial clone / scalar** — monorepo-scale features absent from the current text.
6. **`git maintenance` + commit-graph** as a first-class performance story.
7. **The `git history` command** (2.54+) — a simpler, branch-aware alternative to interactive rebase for rewording, splitting, fixing up, and dropping commits. Likely stabilized by 3.0; a strong candidate to teach as the default before diving into `rebase -i`. (Detailed under Ch 7 above.)

---

## Suggested sequencing

1. **Lock two policy decisions first** (they gate everything):
   (a) `master`→`main` everywhere; (b) how to represent hashes — regenerate all
   examples at SHA-256, or keep labeled SHA-1 legacy examples. These ripple through 600+ edits.
2. **Cross-cutting sweeps** (X1–X3) before per-chapter prose.
3. **High-churn 3.0 chapters**: Ch 10 (internals), Ch 7 (signing/replace), Ch 8 (config), Ch 1 (install/Rust).
4. **Staleness rewrites**: Ch 4 (server), Ch 6 (GitHub), Ch 9 (other SCMs) — most judgment-heavy; do after earlier passes settle.
5. **Appendices** last (fastest to rot; do near publication).
6. **Time the release** to the LTS tag (last 2.x before 3.0): get the manuscript 3.0-ready but hold final publish until the tag lands, so a slip into 2027 doesn't strand the edition.

---

## Companion documents

- [`book_master_to_main_inventory.md`](book_master_to_main_inventory.md) — every `master`
  occurrence, classified (rename / verify-URL / intentional), with per-file counts and a
  line-level checklist.
