# Pro Git, 3rd Edition — Outline & Punch List

The canonical section-by-section punch list for the 3rd-edition restructure.
Content-level staleness and Git 3.0 details live in
[`REVISION_PLAN.md`](REVISION_PLAN.md); this file tracks the new shape of the book and
execution. Each chapter heading notes which current files the chapter is built from.

Chapter order (Git at Scale is second-to-last, before Internals):

> Preface · 1 Getting Started · 2 Git Basics · 3 Git Branching · 4 Distributed Git ·
> 5 Git Toolkit · 6 Customizing Git · 7 Git and Agents · 8 Git Servers ·
> 9 Git at Scale · 10 Git Internals · Appendix A: Git Commands

**Effort** is the writing/editing size for that section (Minimal = mechanical sweeps
only; Small = targeted edits; Medium = significant additions or restructuring;
Large = new or mostly-new material). Update **Status** as work proceeds
(`not started` → `in progress` → `drafted` → `done`).

## Preface

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| P | Preface | The Git 3.0 moment (SHA-256, reftable, `main`, Rust); new-defaults-first stance with dual-path coverage; what's new this edition (scale, agents, worktrees); experimental-command caveats; decide fate of `introduction.asc` (recommend folding in) | Large | not started |

## Chapter 1 — Getting Started

*From `book/ch01/`, all seven sections retained.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 1.0 | Chapter intro & Summary | Wrapper prose and chapter summary refresh | Minimal | not started |
| 1.1 | About Version Control | Local vs. centralized vs. distributed version control, why DVCS won | Minimal | not started |
| 1.2 | A Short History of Git | BitKeeper origins and the kernel's needs; extended with the 2.x→3.0/Rust arc | Small | not started |
| 1.3 | What is Git? | Snapshots, three states, integrity; hash passage rewritten around "SHA-256 by default, SHA-1 in older repos" | Medium | not started |
| 1.4 | The Command Line | Why the book teaches the CLI | Minimal | not started |
| 1.5 | Installing Git | Per-platform install refresh; building from source now requires the Rust toolchain | Medium | not started |
| 1.6 | First-Time Git Setup | Identity, editor, config levels; default-branch subsection flips to "`main` is the default; here's how to override" | Medium | not started |
| 1.7 | Getting Help | `git help`/`-h`; community channels refreshed (Stack Overflow, Discussions, Discord; IRC demoted) | Small | not started |

## Chapter 2 — Git Basics

*From `book/ch02/`; 2.7 expands `ch02/aliases.asc` to deliver the "basic config" promise.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 2.0 | Chapter intro & Summary | Wrapper prose and chapter summary refresh | Minimal | not started |
| 2.1 | Getting a Git Repository | `git init` (what 3.0 creates: `main`, SHA-256, reftable) and `git clone`; `git://` demoted to legacy | Medium | not started |
| 2.2 | Recording Changes to the Repository | Status/add/diff/commit/rm/mv lifecycle; keeps ignoring-files basics (depth moves to 6.2) | Small | not started |
| 2.3 | Viewing the Commit History | `git log`, formatting, filtering; `.mailmap` note | Small | not started |
| 2.4 | Undoing Things | Reframed around `restore`/`switch` as primary verbs; `checkout`/`reset` as legacy forms; amend | Medium | not started |
| 2.5 | Working with Remotes | Remote add/fetch/pull/push/inspect/rename; `pull.rebase` normalized as baseline | Small | not started |
| 2.6 | Tagging | Annotated vs. lightweight tags, sharing and checking out tags | Minimal | not started |
| 2.7 | Basic Configuration and Aliases | Everyday config in one place (identity recap, editor, useful knobs) plus aliases; pointer to Ch 6 for depth | Medium | not started |

## Chapter 3 — Git Branching

*From `book/ch03/` plus one net-new section (3.5 Worktrees — zero existing coverage
anywhere in the book; placeholder at `ch03/worktrees.asc`).*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 3.0 | Chapter intro & Summary | Wrapper prose; summary gains worktrees | Small | not started |
| 3.1 | Branches in a Nutshell | Commit/pointer model, HEAD; `git switch` taught first, `checkout` legacy; all diagrams regenerated for `main` | Medium | not started |
| 3.2 | Basic Branching and Merging | Hotfix/topic walk-through, merge conflicts; strategy is `ort` | Small | not started |
| 3.3 | Branch Management | Listing, merged/unmerged filters; renaming discussion revisited for a main-default world | Small | not started |
| 3.4 | Branching Workflows | Long-running and topic branches; adds GitHub Flow, trunk-based development, stacked branches/PRs | Medium | not started |
| 3.5 | Worktrees | New section: `worktree add`/`list`/`remove`/`lock`; hotfix-mid-feature, parallel builds, one-checkout-per-agent (pointer to Ch 7); new diagrams | Large | not started |
| 3.6 | Remote Branches | Remote-tracking branches, pushing, tracking setup, deleting | Small | not started |
| 3.7 | Rebasing | Basic rebase, `--onto`, the perils, rebase-vs-merge | Small | not started |

## Chapter 4 — Distributed Git

*Was `book/ch05/`, now `book/ch04/`, plus material rescued from the dropped GitHub
chapter into 4.3 (placeholder at `ch04/pull-requests.asc`). Rescued prose (from the
removed `ch06/2-contributing.asc` and `ch06/3-maintaining.asc`; recover via git
history) needs human rewriting to be forge-neutral, not just moving.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 4.0 | Chapter intro & Summary | Wrapper prose; the "next chapter: GitHub" handoff rewritten | Small | not started |
| 4.1 | Distributed Workflows | Centralized, integration-manager, dictator-lieutenants; adds trunk-based development and situates forge/PR collaboration | Medium | not started |
| 4.2 | Contributing to a Project | Commit guidelines; private small/managed teams; email workflow reframed as the kernel/git.git niche; `range-diff`; IMAP-password fix | Medium | not started |
| 4.3 | Pull Requests and Forges | New section: fork → branch → PR → review → merge, forge-neutral; fork syncing and `refs/pull/*` rescued from the GitHub chapter; merge strategies; `gh` CLI | Large | not started |
| 4.4 | Maintaining a Project | `apply`/`am`, contributor branches, determining what's introduced, integrating, tagging releases; adds PR-based maintenance counterpart | Medium | not started |

## Chapter 5 — Git Toolkit

*Was `book/ch07/`, now `book/ch05/`, renamed from "Git Tools."
(`subtree-merges.asc` has no section row of its own: it is included from within
`advanced-merging.asc` as a subsection and stays there.)*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 5.0 | Chapter intro & Summary | Wrapper prose and chapter summary refresh (renamed from "Git Tools") | Minimal | not started |
| 5.1 | Revision Selection | Single revisions, short SHAs (64-char), ancestry, ranges, reflog | Minimal | not started |
| 5.2 | Interactive Staging | `add -i`/`-p`, staging hunks | Minimal | not started |
| 5.3 | Stashing and Cleaning | Stash/apply/pop/branch, `clean`; `stash save` → `stash push`; `show -u` | Small | not started |
| 5.4 | Signing Your Work | Adds SSH signing (`gpg.format=ssh`) as the mainstream path alongside GPG; modern keys; verification; forge display | Large | not started |
| 5.5 | Searching | `git grep`, `log -S`/`-G`, line-log `-L` | Minimal | not started |
| 5.6 | Rewriting History | Amend, rebase -i, autosquash; adds `git history` (reword/split/fixup/drop) as the everyday tool; `filter-repo` leads, `filter-branch` demoted | Large | not started |
| 5.7 | Reset Demystified | The three trees, reset vs. checkout, path forms; cross-referenced with `restore` | Minimal | not started |
| 5.8 | Advanced Merging | Conflict tooling, ours/theirs, criss-cross; `ort` naming; `merge-tree` note | Small | not started |
| 5.9 | Rerere | Reuse recorded conflict resolutions | Minimal | not started |
| 5.10 | Debugging with Git | `blame`, bisect; adds `git bisect run` | Small | not started |
| 5.11 | Submodules | Adding, updating, publishing, nesting; "newer Git" framing normalized (heaviest mechanical file: 35 renames) | Small | not started |
| 5.12 | Bundling | `git bundle` for offline transfer | Minimal | not started |
| 5.13 | Replace | Reworked: grafts removed in 3.0; taught around `replace` incl. `--graft` | Medium | not started |
| 5.14 | Credential Storage | Cache/store/osxkeychain; Git Credential Manager as the cross-platform standard; custom helpers | Medium | not started |

## Chapter 6 — Customizing Git

*Was `book/ch08/`, now `book/ch06/`; 6.2 (placeholder at `ch06/ignoring.asc`) is
assembled from `ch02/recording-changes.asc` (`_ignoring`) and `config.asc` excludes
material.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 6.0 | Chapter intro & Summary | Wrapper prose and chapter summary refresh | Minimal | not started |
| 6.1 | Git Configuration | Config levels + XDG path; adds `includeIf`, `core.hooksPath`, `safe.directory`/`safe.bareRepository`, `init.defaultBranch`, 3.0 extension knobs | Large | not started |
| 6.2 | Ignoring Files | Promoted section: pattern rules in depth, nested `.gitignore`, global excludes, `check-ignore`; assembled from Ch 2 + config material | Medium | not started |
| 6.3 | Git Attributes | Diff/merge drivers, smudge/clean filters, `export-ignore`; LFS cross-ref to 9.6 | Small | not started |
| 6.4 | Git Hooks | Hook catalog refreshed with post-2014 hooks; shared hooks via `core.hooksPath`; hook managers note | Small | not started |
| 6.5 | An Example Git-Enforced Policy | Server-side enforcement capstone (update hook + client mirror); open decision: keep (recommended) or cut | Minimal | not started |

## Chapter 7 — Git and Agents

*Entirely new chapter (placeholders at `book/ch07-git-and-agents.asc` + `book/ch07/`);
7.3 is a condensed reflow of the dropped Appendix B (`book/B/`, removed — recover via
git history) plus gitoxide and per-library SHA-256 notes.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 7.0 | Chapter intro & Summary | New chapter: why AI coding agents change repo usage — many actors, high commit volume, review as bottleneck | Medium | not started |
| 7.1 | Workflows for Agents | Branch-per-agent, worktree-per-agent (builds on 3.5), sandboxed checkouts, commit/push cadence, draft PRs as hand-off | Large | not started |
| 7.2 | Tools and Interfaces | How agents drive Git: the CLI as universal interface, `gh`, MCP servers, forge APIs and webhooks | Large | not started |
| 7.3 | Libraries | Programmatic Git: libgit2 (+ bindings), JGit, go-git, Dulwich, gitoxide; SHA-256 support per library (condensed reflow of old Appendix B) | Medium | not started |
| 7.4 | Guiding Agents | Repo-level agent instructions (AGENTS.md-style), skills, commit/branch conventions for machine contributors, ignoring agent artifacts | Large | not started |
| 7.5 | Tips and Pitfalls | Reviewing agent-authored history, attribution trailers, safety rails (protected branches, force-push, `safe.*`), secrets hygiene | Medium | not started |

## Chapter 8 — Git Servers

*Was `book/ch04/`, now `book/ch08/`; 8.8 is reworked from `gitlab.asc`, 8.9 from
`hosted.asc` (currently a GitHub/Bitbucket-era stub that deferred to the dropped
GitHub chapter).*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 8.0 | Chapter intro & Summary | Wrapper prose and chapter summary refresh | Small | not started |
| 8.1 | The Protocols | Local, SSH, smart HTTP (protocol v2) as the living protocols; dumb HTTP cut to history; `git://` legacy | Medium | not started |
| 8.2 | Getting Git on a Server | Bare repositories, `--shared`, putting a repo on a box | Minimal | not started |
| 8.3 | Generating Your SSH Public Key | Ed25519 as the recommended type; DSA-era advice removed | Small | not started |
| 8.4 | Setting Up the Server | The `git` user, `authorized_keys`, `git-shell`; xinetd/sysvinit dropped | Small | not started |
| 8.5 | Git Daemon | Trimmed to a short treatment (systemd unit), framed as niche | Small | not started |
| 8.6 | Smart HTTP | `git-http-backend` behind current Apache/nginx config; token/OIDC auth notes | Medium | not started |
| 8.7 | GitWeb | Open decision: cut to a sidebar mention (recommended) or keep trimmed | Small | not started |
| 8.8 | Self-Hosted Forges | Reworked from GitLab-only: Gitea/Forgejo as the lightweight lead, GitLab heavyweight; install sketch + day-one basics; new screenshots | Large | not started |
| 8.9 | Hosted Options | The hosting landscape (GitHub, GitLab.com, Codeberg, Sourcehut, Azure DevOps) and choosing hosted vs. self-hosted | Medium | not started |

## Chapter 9 — Git at Scale

*Entirely new chapter (placeholders at `book/ch09-git-at-scale.asc` + `book/ch09/`) —
no existing coverage of shallow/partial clones, sparse checkout, Scalar, LFS, or
monorepos anywhere in the current book. 9.5 keeps the ops-level performance story;
on-disk format detail stays in Chapter 10.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 9.0 | Chapter intro & Summary | New chapter: why big repos and teams strain Git; map of the scaling toolbox | Medium | not started |
| 9.1 | Shallow Clones | `--depth`, `--shallow-since`, deepening/unshallowing, CI patterns, limitations | Medium | not started |
| 9.2 | Partial Clone | Blob/tree filters, promisor remotes, lazy on-demand fetch, `git backfill`; when partial beats shallow | Large | not started |
| 9.3 | Sparse Checkout | `git sparse-checkout`, cone mode, sparse index; working in a directory subset | Medium | not started |
| 9.4 | Scalar | `scalar clone`/`register` as the batteries-included front door; what it enables | Medium | not started |
| 9.5 | Keeping Large Repositories Fast | `git maintenance` schedules, commit-graph, multi-pack-index, FSMonitor (ops story; formats stay in Ch 10) | Medium | not started |
| 9.6 | Large Files | Git LFS basics; when to prefer it vs. partial clone; keeping binaries out of history | Medium | not started |
| 9.7 | Monorepos | Putting it together: trade-offs, layout/ownership practices, real-world Scalar deployments, forge constraints | Large | not started |

## Chapter 10 — Git Internals

*From `book/ch10/`, all eight sections retained.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| 10.0 | Chapter intro & Summary | Wrapper prose ("last chapter" framing still true) and summary refresh | Minimal | not started |
| 10.1 | Plumbing and Porcelain | The `.git` directory tour, updated for a 3.0 repo (reftable dirs, no legacy `branches/`) | Small | not started |
| 10.2 | Git Objects | Blobs/trees/commits retaught with object-format framing: SHA-256 default + SHA-1 interop; walk-throughs regenerated | Large | not started |
| 10.3 | Git References | Refs/HEAD/tags/remotes plus a substantial new reftable subsection (format and why); loose/packed-refs reframed as legacy | Large | not started |
| 10.4 | Packfiles | Pack/idx formats and deltification; adds multi-pack-index and commit-graph format detail | Medium | not started |
| 10.5 | The Refspec | Fetch/push refspec syntax; PR-refs cross-reference retargeted to 4.3 | Small | not started |
| 10.6 | Transfer Protocols | Rebuilt around protocol v2 (ref filtering, negotiation); dumb protocol shrunk to a historical aside | Large | not started |
| 10.7 | Maintenance and Data Recovery | `git maintenance` as the modern face of `gc`; reflog/`fsck` recovery; big-file removal redone with `filter-repo` | Medium | not started |
| 10.8 | Environment Variables | The environment-variable survey | Minimal | not started |

## Appendix A — Git Commands

*Was `C-git-commands.asc`, now `A-git-commands.asc` — the only appendix kept.*

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| A | Git Commands | Command index retargeted to the new chapter map; adds `switch`, `restore`, `worktree`, `sparse-checkout`, `scalar`, `maintenance`, `range-diff`, `history`; notes 3.0 removals; External Systems category trimmed; broken xrefs fixed | Medium | not started |

## Dropped outright

Numbering below is the *2nd edition's*; section numbers in the right column are the new outline's.

| Dropped (2nd ed.) | Where its survivors go |
|---|---|
| Ch 6 GitHub (`book/ch06/`) | PR flow, fork syncing, PR refs → **4.3/4.4**; account setup, org management, Markdown, scripting/API → cut |
| Ch 9 Git and Other Systems (`book/ch09/`) | Cut entirely (SVN/Hg/P4 bridges, importers); command-index entries trimmed to match |
| Appendix A Git in Other Environments (`book/A/`) | Cut entirely (GUIs, IDEs, shell prompts) |
| Appendix B Embedding Git (`book/B/`) | Library survey reflows, condensed, into **7.3** |
| `introduction.asc` | Open decision: fold into the Preface or Ch 1 opener; recommend folding into the Preface |

## Open decisions

1. `policy.asc` (6.5) — keep (recommended) or cut.
2. GitWeb (8.7) — cut to a sidebar mention (recommended) or keep trimmed.
3. `introduction.asc` — fold into the Preface (recommended) or keep as a standalone opener.
4. Large Files (9.6) — lives in Git at Scale (recommended) vs. Git Servers.
5. Performance split — ops story in 9.5, format detail in 10.4/10.7 (recommended), or consolidate in one place.

## Cross-cutting & mechanical

| Section | Title | Description | Effort | Status |
|---|---|---|---|---|
| X1 | master → main sweep | ~602 renames book-wide per `book_master_to_main_inventory.md`; coordinate with diagram regeneration | Large | not started |
| X2 | SHA-256 example hashes | Regenerate example hashes/transcripts book-wide at 64 chars; settle the legacy-SHA-1-example policy | Large | not started |
| X3 | Version reframing | "Written for Git 2" → 3.x; normalize "since 2.x" framing to baseline behavior | Medium | not started |
| X4 | Restructure plumbing | New include map in `progit.asc`; wrapper files for Ch 7 and Ch 9; delete `ch06/`, `ch09/`, `A/`, `B/` and their images; renumber chapter dirs | Medium | done |
| X5 | Cross-reference repair | Retarget xrefs into dropped chapters (`refspec.asc`, `A-git-commands.asc`, `contributing.asc`, `hosted.asc`) — done via anchors preserved in the 4.3 placeholder; Ch 2's `_ignoring` anchor moves with the 6.2 content work | Small | done |
| X6 | Diagram regeneration | `diagram-source/` regenerated for `main`; new diagrams for 3.5 (worktrees), Ch 7, and Ch 9 | Large | not started |
