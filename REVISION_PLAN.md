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
- 🕰 `installing.asc`: version note (X3); refresh per-platform install steps to Windows 10/11 and macOS 12+; remove all references to Windows XP/Vista and macOS Mavericks. **Add the Rust toolchain requirement** — 3.0 makes Rust mandatory, so "compile from source" must cover installing Rust/cargo.
- 🕰 `first-time-setup.asc:86-96`: rewrite the default-branch subsection — flips from "how to change it" to "it's `main`; how to override if needed." Remove the 32-bit Notepad++ caveat (long obsolete).
- 🕰 `first-time-setup.asc` / `generating-ssh-key.asc`: `id_dsa` key type mentioned as an option — DSA is disabled in OpenSSH 7+ defaults; replace with `id_ed25519` as the recommended type throughout.
- 🕰 Help resources: IRC (`#git` on Libera Chat) listed as a primary channel — still exists but is secondary; add Stack Overflow, GitHub Discussions, and Discord as primary resources.
- ⚡ `what-is-git.asc:51-63`: the headline SHA-1 passage (X2).

### Ch 2 — Git Basics
- 🕰 `undoing.asc:153-155`: `restore`/`switch` framed as "new in 2.23" — normalize as standard; consider making them the primary taught commands over `checkout`/`reset`.
- 🕰 `remotes.asc:132`: `pull.rebase` warning framed as "since 2.27."
- 🕰 `remotes.asc` / `getting-a-repository.asc`: `git://` protocol shown as a normal option — largely blocked by firewalls and unauthenticated; mark as legacy and recommend HTTPS or SSH only.
- 🕰 `stashing-cleaning.asc`: `git stash save` is deprecated in favor of `git stash push` (since 2.16) — update all examples.
- 🕰 `viewing-history.asc`: `git log` now applies `.mailmap` by default (since 2.23) — add a note that author names and emails can be normalized via `.mailmap`.
- 🕰 `recording-changes.asc`: `git add :(attr:...)` magic pathspec supported since 2.44 — mention in the pathspec coverage.
- ⚡ Example hashes throughout (`recording-changes` 35 refs, `tagging`, `viewing-history`) are SHA-1 (X2).
- ⚡ `getting-a-repository.asc`: `git init` now yields `main` and (3.0) a reftable backend — add a forward-reference note.

### Ch 3 — Git Branching
- ⚡ **Heaviest `master` concentration** (101 renames): `basic-branching-and-merging` (22), `rebasing` (24), `remote-branches` (18), `branch-management` (18), `nutshell` (15). All canonical branch diagrams say `master` — `diagram-source/` needs regenerating.
- 🕰 Content is solid; mostly the X1 sweep + diagram regeneration.
- ⚡ `branch-management.asc:82,131` already discuss `master/main/mainline` renaming — keep the concept, revisit wording for a main-default world.
- 🕰 `nutshell.asc`: add `git switch` as the primary branch-switching verb and `git switch -c` as the primary branch-creation form; note `git checkout`/`git checkout -b` as legacy equivalents.
- 🕰 `basic-branching-and-merging.asc`: merge strategy described as "recursive" — changed to "ort" (Optimal Resolution Tree) in 2.34; update and note that ort is faster and more correct on complex merges.
- 🕰 `rebasing.asc`: `git rebase` switched to merge-based backend by default in 2.26 — note the behavioral baseline and remove any framing that treats it as new.
- 🕰 `workflows.asc`: covers only long-running + topic branches; add GitHub Flow, trunk-based development, and stacked diffs/PRs as modern patterns.
- 🕰 Add a brief `git worktree` introduction in the branching context (`worktree add --orphan` arrived in 2.42); full coverage belongs in Ch 7.

### Ch 4 — Git on the Server
- 🕰 **Most dated infrastructure chapter.** `git-daemon`, `gitweb`, and hand-rolled `setting-up-server` describe near-unused practices. Demote Gitweb/daemon; lead with modern self-hosting. **Gitea/Forgejo is not mentioned at all** and should be added.
- 🕰 `protocols`/`smart-http`: dumb HTTP is effectively dead; **protocol v2** (default since 2.26) needs proper coverage.
- 🕰 `generating-ssh-key.asc`: recommend **Ed25519** as default. Remove the `-o` flag caveat — the new OpenSSH key format has been the default since OpenSSH 7.8 (2019). Replace any `id_dsa` references. Remove `authorized_keys2` — merged into `authorized_keys` since OpenSSH 3.9.
- 🕰 `setting-up-server.asc`: remove `xinetd`/`sysvinit` service management for `git-daemon` — essentially gone from modern Linux; footnote only if retained at all.
- 🕰 `smart-http.asc`: Apache `order allow,deny` / `Allow from all` syntax is deprecated since Apache 2.4 (2012) — update to `Require all granted`. `.htpasswd` for auth is insecure for public-facing services; add a note on OAuth/OIDC/token-based alternatives.
- 🕰 `hosted.asc`: hosted services section lists only Bitbucket and GitHub; update to include GitLab.com, Gitea/Forgejo (self-hosted open source), Codeberg, Sourcehut, and Azure DevOps with a brief "who self-hosts and why" comparison.
- 🕰 Add a **Git LFS** section — critical for large binary file workflows and entirely absent from the current book.
- 🕰 Add a **partial clones / shallow clones** section (`--filter=blob:none`, `--depth`, etc.) — now mainstream for large repos. Note `git clone --no-tags` (since 2.14) as a related option for avoiding tag bloat.
- ⚡ Sidebar on reftable + SHA-256 hosting/interop implications (forge readiness is what gates 3.0's date).

### Ch 5 — Distributed Git
- 🕰 X1 sweep: `contributing` (48 renames), `maintaining` (39). Content (contributing workflows, `format-patch`/`am`, integration-manager model) is durable.
- 🕰 Contextualize email-based workflow against PR-based norms; still valid for kernel/Git communities. Gmail IMAP configuration in the book stores a plaintext password in the config file — security anti-pattern; add credential helper guidance and note Gmail now requires app-specific passwords.
- 🕰 `contributing.asc`: `git request-pull` presented without context — rarely used outside kernel/old-school projects; add a note that this is a niche workflow.
- 🕰 Add a section or sidebar on the **`gh` CLI** for PR creation and management — now widely used for forge-based contributing workflows.
- 🕰 Add brief coverage of **CI/CD integration** into distributed workflows (how CI status checks tie into branch/PR merge decisions).
- 🕰 `distributed-workflows.asc`: add **trunk-based development** alongside the existing integration-manager and dictator-lieutenant models.
- 🕰 `contributing.asc`: `git format-patch --interdiff` / `--range-diff` (since 2.19/2.20) are useful for explaining changes between patch series versions — reference `git range-diff` coverage from Ch 7 or add a brief treatment here.

### Ch 6 — GitHub
- 🕰 **Fastest-rotting chapter.** All screenshots stale; PR review UI, org settings, account setup flows all changed. Full re-capture + text pass.
- 🕰 **Structural decision (open):** The chapter is large enough to split — "GitHub for Collaboration" (PRs, issues, forks, reviews) and "Automating GitHub" (Actions, Apps, webhooks, API). Decide before writing begins; a split affects cross-references and the TOC.
- 🕰 `1-setting-up-account.asc`: 2FA described as TOTP/SMS only — WebAuthn/passkeys are now GitHub's preferred 2FA (2022+) and SMS is discouraged; update. Remove Gravatar reference; GitHub now uses its own avatar system.
- 🕰 `5-scripting.asc`: basic auth for API access was removed by GitHub in 2020 — replace with PATs and OAuth flows. API rate limits (60/5000 req/hr) have changed and now depend on auth method; update. Remove "Services" (GitHub service hooks) — deprecated ~2018 and removed; replace with GitHub Apps and webhooks.
- 🕰 Add a **GitHub Apps** section — now the recommended integration method (introduced 2016), superseding OAuth apps and service hooks for automation.
- 🕰 Add a **GitHub Actions** section — the entire CI/CD automation story changed in 2019; this is the most important omission in the chapter. Cover workflows, triggers, jobs, and the marketplace at a minimum.
- 🕰 `5-scripting.asc`: REST API only — add a note on the GraphQL API (introduced 2016, now recommended for many use cases).
- 🕰 Add a **`gh` CLI** section — first released 2020, now widely used for PR, issue, and release workflows from the terminal.
- 🕰 `2-contributing.asc`: merge button shown with merge-commit only — squash-merge and rebase-merge were added ~2016; update. Add **branch protection rules** (critical modern feature, ~2016) and **CODEOWNERS** (added 2017, widely used).
- 🕰 Mention **Dependabot** (built-in dependency security), **GitHub Discussions** (2020), **GitHub Packages** (2019), and **GitHub Copilot / AI features** (2022+; at minimum acknowledge the category).
- 🕰 `5-scripting.asc`: Octokit listed as supporting Go, Objective-C, Ruby, .NET only — vastly expanded; official SDKs now exist for many languages; update the survey.
- 🕰 Missing modern surface area: **Codespaces, current PR review experience.** Scope decision — chapter is deliberately "GitHub as an example forge," not exhaustive.
- ⚡ Default-branch language in examples (26 renames in `2-contributing`).

### Ch 7 — Git Tools
- ⚡ `signing.asc`: **GPG-only** today. Add **SSH commit/tag signing** (`gpg.format=ssh`, since 2.34) — now the mainstream choice — and `gpgsm` (X.509). Significant content addition. GPG key examples show 2048-bit RSA — update to RSA 4096 or Ed25519 as the modern recommendation.
- ⚡ `replace.asc`: contains the book's only **grafts** discussion (17 `master` refs too) — grafts are **removed in 3.0**. Rework around `replace`/`commit-graph`; mark grafts removed.
- ⚡ `rewriting-history.asc`: add the new **`git history`** command (experimental, introduced Git 2.54 / April 2026; `fixup` added 2.55). It rewrites history by modifying specific commits and **automatically rebases descendant branches** — a much simpler mental model than interactive rebase. Cover its four subcommands: **`reword`** (change a commit message in place), **`split`** (interactively carve one commit into two by hunk), **`fixup`** (fold staged changes into an older commit via three-way merge), **`drop`** (remove a commit, replaying descendants onto its parent). Note the current limitations: experimental/behavior-may-change, no merge commits, no operations that would produce conflicts, cannot drop root/merge commits. Position it alongside interactive rebase as the recommended everyday tool for the common cases.
- ⚡ `rewriting-history.asc`: `git rebase --autosquash` now works for non-interactive rebase (since 2.44; incompatible with apply backend) — update autosquash coverage to reflect this.
- 🕰 `credentials.asc`: **Git Credential Manager (GCM / `manager`)** is now the cross-platform standard — update legacy `wincred`/naming. macOS `osxkeychain` still fine.
- 🕰 `rewriting-history.asc`: `filter-branch` is deprecated and warns on use; lead with **`git filter-repo`** (note BFG). Add coverage of **`git bisect run`** (automated bisect using a test script) — currently underexplored.
- 🕰 `rewriting-history.asc`: `git rebase` auto-skip of commits equivalent to existing history (since 2.34) — describe as baseline behavior, not a new feature.
- 🕰 `stashing-cleaning.asc`: `git stash save` is deprecated in favor of `git stash push` — update all examples. Note `git stash show --untracked` (since 2.32).
- 🕰 `submodules.asc`: remove "newer versions" framing for Git 2.12-2.14 behaviors — these are 7+ years old; describe as baseline.
- 🕰 Highest total `master` count of any chapter (147 renames): `submodules` (35), `revision-selection` (20), `advanced-merging` (17), `bundling` (14), `stashing-cleaning` (14), `subtree-merges` (13).

### Ch 8 — Customizing Git
- ⚡ `config.asc`: add **`safe.bareRepository`** and **`safe.directory`** (X4); `init.defaultBranch` as default-is-main; note SHA-256 (`--object-format`) and reftable (`extensions.refStorage`).
- 🕰 `config.asc`: add **`includeIf`** for conditional configuration (since 2.13) — profile-switching between work and personal repos is a very common modern pattern.
- 🕰 `config.asc`: **`core.hooksPath`** (since 2.9) not mentioned — critical for centralized/shared hooks management; add. Survey missing modern hook types added post-2014: `push-to-checkout`, `reference-transaction`, etc.
- 🕰 `config.asc`: **XDG Base Directory** config path (`~/.config/git/config`) should be given equal prominence to `~/.gitconfig` — preferred on Linux/macOS when `~/.gitconfig` is absent (XDG write priority finalized in 2.44).
- 🕰 `config.asc`: **`url.<base>.insteadOf`** for rewriting clone URLs (e.g., SSH instead of HTTPS in CI) — consider adding.
- 🕰 `config.asc`: author/committer config overrides (`author.name`, `author.email`, `committer.name`, `committer.email`, since 2.22) override `user.*` settings — useful for per-repo identity; mention alongside `user.name`/`user.email`.
- 🕰 `hooks`, `attributes`, `policy` durable. `policy.asc` uses `master` (8×) in its enforced-workflow example.

### Ch 9 — Git and Other Systems
- 🕰 **Structural decision (open):** Consider renaming to "Git and Legacy SCMs" or "Migrating to Git" — the chapter's purpose has shifted from ongoing interop to one-time migration, and the title should reflect that.
- 🕰 **Strongest candidate for deep cuts.** `git svn` retains users (trim). The **Mercurial** bridge (`client-hg`/`import-hg`, 23+ `master` refs) is largely unmaintained and Bitbucket dropped Hg hosting in 2020; **Perforce** (`git-p4`, 37 refs) is niche. Recommend: keep trimmed `git svn`, demote Hg/P4 to a short "bridges exist" section, lean on generic `import-custom.asc` fast-import.
- 🕰 `perforce.asc`: add scope context — Perforce remains relevant in game development and some large enterprises; frame accordingly.
- 🕰 Add a section on **forge-to-forge migration** (GitHub → GitLab, GitLab → Gitea/Forgejo, etc.) — now a common scenario entirely absent from the chapter.
- 🕰 Verify `git-p4`/`hg` tooling runs on modern Python (scripts predate the Python-2 sunset).

### Ch 10 — Git Internals
- ⚡ **The chapter most reshaped by 3.0** (70 renames).
  - `objects`/`packfiles`: object model taught as SHA-1 20-byte / 40-hex (X2) — needs a SHA-256 rewrite with an "object format" framing and the interop story.
  - `refs.asc:7-23`: teaches refs purely as loose files under `.git/refs` + packed-refs. 3.0 makes **reftable** the default — substantial addition needed (and the _why_: Windows/macOS case-collision + performance).
  - `transfer-protocols.asc`: fold in **protocol v2**. Shrink "dumb protocol" coverage significantly — essentially unused; even GitHub disabled it years ago.
- 🕰 `maintenance.asc`: `git maintenance` relatively current; verify against latest.
- 🕰 `packfiles.asc`: Pack v2 format not mentioned — add a brief note (improves performance with many refs).
- 🕰 Add **`commit-graph`** coverage (since 2.18) — dramatically speeds up graph traversal; `git maintenance` writes it automatically; pairs with the performance story.
- 🕰 Add **partial clone internals** (promised objects, lazy-loading blobs, filter specs) — pairs with the partial clone sections in Ch 4 and Ch 7.

### Appendix A — Git in Other Environments
- 🕰 Editor/IDE coverage rots fast: `sublimetext`, `visualstudio`, `visualstudiocode`, `jetbrainsides` need version/screenshot refresh. `visualstudiocode.asc:5` still says "Git 2.0.0 or newer." Shell completion (`bash`/`zsh`/`powershell`) stable — version-check.
- 🕰 `sublimetext.asc`: Sublime Text is now a minority editor relative to VS Code; consider shrinking to a short mention.
- 🕰 GUI client survey is stale — GitKraken, GitHub Desktop, Fork, and Sourcetree have all evolved significantly; update.
- 🕰 Shell/terminal integrations: Fish shell (popular on macOS/Linux) and **Starship** prompt are widely used but not mentioned; add a section alongside `bash`/`zsh`.
- 🕰 Add a category for **AI-assisted Git tools** (GitHub Copilot CLI, AI-powered commit message generation, gitlint integrations) — at minimum acknowledge the category.

### Appendix B — Embedding Git
- 🕰 Binding versions drift: `libgit2`, `jgit`, `go-git`, `dulwich`. **Promote `go-git`** (now a mainstream pure-Go implementation). ⚡ Check each binding's **SHA-256 support status** — a real reader concern during the transition; add a per-library compatibility note.
- 🕰 `libgit2.asc`: bindings listed as Ruby (Rugged) only — expand to cover Rust (`git2-rs`), Go (`git2go`), Python (`pygit2`), and Node (`nodegit`/`libgit2.js`).
- 🕰 Add a **`gitoxide`** section — a significant new Git implementation in Rust (`gix` crate), gaining traction as both a library and a CLI tool; not mentioned anywhere in the current book.

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
| `git merge-tree` | 2.38 (2022) | stable | Ch 7 — server-side merge computation without a working tree; `--merge-base` option added 2.40 |
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
8. **`git merge-tree`** (2.38+) — server-side merge computation without a working tree; lets tooling check mergeability without a checkout. Ch 7.
9. **Stacked diffs / stacked PRs** — emerging collaboration pattern (Graphite, ghstack, etc.); pairs with the distributed workflows update in Ch 5.

---

## Suggested sequencing

0. **Resolve open structural decisions first** (flagged inline above):
   (a) Split Ch 6 into "GitHub for Collaboration" + "Automating GitHub" or expand as one chapter.
   (b) Rename Ch 9 to "Git and Legacy SCMs" / "Migrating to Git" or leave as-is.
   These affect the TOC, cross-references, and scope of each chapter's writing work.
1. **Lock two policy decisions** (they gate the bulk of edits):
   (a) `master`→`main` everywhere; (b) how to represent hashes — regenerate all
   examples at SHA-256, or keep labeled SHA-1 legacy examples. These ripple through 600+ edits.
2. **Cross-cutting sweeps** (X1–X3) before per-chapter prose.
3. **High-churn 3.0 chapters**: Ch 10 (internals), Ch 7 (signing/replace), Ch 8 (config), Ch 1 (install/Rust).
4. **Staleness rewrites**: Ch 4 (server), Ch 6 (GitHub), Ch 9 (other SCMs) — most judgment-heavy; do after earlier passes settle.
5. **Appendices** last (fastest to rot; do near publication).
6. **Time the release** to the LTS tag (last 2.x before 3.0): get the manuscript 3.0-ready but hold final publish until the tag lands, so a slip into 2027 doesn't strand the edition.

---

## Effort by chapter

| Chapter | Staleness level | Primary driver | Effort |
|---------|----------------|----------------|--------|
| 1: Getting Started | Low-Medium | Platform refs, SSH key type, SHA-256 intro | Small |
| 2: Git Basics | Medium | switch/restore primary, stash, git:// legacy, hashes | Medium |
| 3: Branching | Medium | X1 + diagram regen; add switch, ort, worktree intro, workflows | Medium |
| 4: Server | High | SSH keys, Apache config, hosting landscape, LFS, partial clones | Large |
| 5: Distributed | Medium-High | Demote email workflow; add PR/CI/gh/trunk-based | Medium |
| 6: GitHub | **Critical — rewrite** | New UI, Actions, Apps, CLI, branch protection, passkeys | Very Large |
| 7: Git Tools | Medium-High | git history, filter-repo, signing, bisect run, stash, submodules | Large |
| 8: Customizing | Medium | includeIf, hooksPath, GCM, XDG, safe.*, committer overrides | Medium |
| 9: Other SCMs | Medium | Deep cuts, forge migration, Python 2 check, possible rename | Medium |
| 10: Internals | High | SHA-256, reftable, commit-graph, partial clone internals, protocol v2 | Large |
| Appendix A | High | IDE/tool screenshots + Fish/Starship/AI tools | Large |
| Appendix B | Medium | Add gitoxide, expand libgit2 bindings, SHA-256 compat notes | Medium |

---

## Companion documents

- [`book_master_to_main_inventory.md`](book_master_to_main_inventory.md) — every `master`
  occurrence, classified (rename / verify-URL / intentional), with per-file counts and a
  line-level checklist.
