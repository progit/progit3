# Pro Git, 3rd Edition — Restructure Outline

Full chapter/section outline for the 3rd-edition restructure. Companion to
[`REVISION_PLAN.md`](REVISION_PLAN.md), which tracks the content-level staleness and
Git 3.0 changes; this document tracks the new *shape* of the book. For each section:
what it covers, and how much of it is new versus carried over.

**Status legend:**

- **new** — section does not exist today; written from scratch.
- **assembled** — new section, but largely built from existing prose moved from elsewhere.
- **reworked** — section exists but its framing/structure changes substantially.
- **largely updated** — section survives but needs significant content additions/changes.
- **slightly updated** — targeted edits (a passage or two, plus mechanical sweeps).
- **not changed much** — only the mechanical sweeps (master→main, hash regeneration, version framing).
- **trimmed** / **cut** — reduced to a short treatment, or removed.

All sections additionally receive the cross-cutting sweeps (X1–X3 in REVISION_PLAN):
`master`→`main`, SHA-256 example hashes, and version reframing. Those are not repeated
per-section below unless they dominate the work.

---

## Preface *(from `book/preface.asc` — placeholder outline exists)*

Covers: the Git 3.0 moment (SHA-256, reftable, `main`, Rust, removals); the book's
stance of treating 3.0 defaults as *the* defaults with dual-path coverage for older
repos; what's new in this edition (scale, agents, worktrees); experimental-command
caveats (`git history`); guidance for readers still on 2.x.

Status: **new** prose. The outline placeholder exists but doesn't yet mention the AI
and worktree themes — extend the outline, then a human writes the prose.

---

## Chapter 1 — Getting Started *(from `book/ch01/`)*

**1.1 About Version Control** (`about-version-control.asc`)
Local vs. centralized vs. distributed version control, and why DVCS won.
Status: **not changed much**.

**1.2 A Short History of Git** (`history.asc`)
BitKeeper origins, the kernel's needs; extended with Git's maturation and the 2.x→3.0
transition (Rust, SHA-256) as the newest chapter of the story.
Status: **slightly updated**.

**1.3 What is Git?** (`what-is-git.asc`)
Snapshots-not-diffs, the three states, local operation, integrity — the hash passage
rewritten around "a cryptographic hash: SHA-256 by default, SHA-1 in older repos."
Status: **largely updated** (the headline SHA-1 passage is the book's marquee X2 edit).

**1.4 The Command Line** (`command-line.asc`)
Why the book teaches the CLI.
Status: **not changed much**.

**1.5 Installing Git** (`installing.asc`)
Per-platform install refresh (drop XP/Mavericks-era steps); building from source now
requires the Rust toolchain (mandatory in 3.0).
Status: **largely updated**.

**1.6 First-Time Git Setup** (`first-time-setup.asc`)
Identity, editor, config levels; the default-branch subsection flips from "how to opt
into `main`" to "`main` is the default; here's how to override."
Status: **largely updated**.

**1.7 Getting Help** (`help.asc`)
`git help`/`-h`; community channels refreshed (Stack Overflow, GitHub Discussions,
Discord promoted; IRC demoted).
Status: **slightly updated**.

**1.8 Summary**
Status: **slightly updated**.

---

## Chapter 2 — Git Basics *(from `book/ch02/`)*

**2.1 Getting a Git Repository** (`getting-a-repository.asc`)
`git init` (what you get under 3.0: `main`, SHA-256, reftable — with forward pointers)
and `git clone`; clone URL forms with `git://` demoted to a legacy note.
Status: **largely updated**.

**2.2 Recording Changes to the Repository** (`recording-changes.asc`)
Status/add/diff/commit/rm/mv lifecycle; keeps the *basics* of ignoring files here, with
the in-depth pattern material moving to Customizing Git (see 7.2).
Status: **slightly updated** (plus every example hash regenerated).

**2.3 Viewing the Commit History** (`viewing-history.asc`)
`git log`, formatting, filtering; add the `.mailmap` normalization note.
Status: **slightly updated**.

**2.4 Undoing Things** (`undoing.asc`)
Reframed around `git restore` (and `switch`) as the primary verbs, with
`checkout`/`reset` as the legacy forms; amend; unmodifying files.
Status: **largely updated** (reframing, not just edits).

**2.5 Working with Remotes** (`remotes.asc`)
Remote add/fetch/pull/push/inspect/rename; `pull.rebase` guidance normalized as
baseline behavior.
Status: **slightly updated**.

**2.6 Tagging** (`tagging.asc`)
Annotated vs. lightweight tags, sharing and checking out tags.
Status: **not changed much**.

**2.7 Basic Configuration and Aliases** (expanded from `aliases.asc`)
The everyday config a new user needs in one place — identity recap, editor, useful
one-liners — plus aliases; points to Customizing Git for depth. Delivers the outline's
"basic config" promise for this chapter.
Status: **assembled** (aliases prose carried over; config prose largely relocated from
Ch 1 / Ch 7 material, with a small amount of new writing).

**2.8 Summary**
Status: **slightly updated**.

---

## Chapter 3 — Git Branching *(from `book/ch03/` + one new section)*

**3.1 Branches in a Nutshell** (`nutshell.asc`)
Commits/pointers model, HEAD, creating and switching — `git switch` taught first,
`checkout` as legacy; all branch diagrams regenerated for `main`.
Status: **largely updated** (switch-first reframe + full diagram regeneration).

**3.2 Basic Branching and Merging** (`basic-branching-and-merging.asc`)
The hotfix/topic-branch walk-through; merge conflicts; merge strategy named `ort`.
Status: **not changed much** narratively, but mechanically heavy (22 renames + diagrams).

**3.3 Branch Management** (`branch-management.asc`)
Listing, filtering merged/unmerged, renaming; the branch-renaming discussion revisited
for a world where `main` is the default.
Status: **slightly updated**.

**3.4 Branching Workflows** (`workflows.asc`)
Long-running branches and topic branches; adds GitHub Flow, trunk-based development,
and stacked branches/PRs as modern patterns.
Status: **largely updated** (new subsections).

**3.5 Worktrees** — *new section*
What a worktree is; `git worktree add`/`list`/`remove`/`lock`; use cases: hotfix while
mid-feature, parallel builds/tests, one-checkout-per-agent (forward pointer to Ch 8);
orphan worktrees. New diagrams.
Status: **new** (zero existing coverage anywhere in the book).

**3.6 Remote Branches** (`remote-branches.asc`)
Remote-tracking branches, pushing, tracking setup, pulling, deleting remote branches.
Status: **not changed much** narratively; heavy mechanical sweep + diagrams.

**3.7 Rebasing** (`rebasing.asc`)
Basic rebase, `--onto`, the perils, rebase-vs-merge philosophy.
Status: **not changed much** narratively; heavy mechanical sweep + diagrams.

**3.8 Summary**
Status: **slightly updated** (worktrees added to the recap).

---

## Chapter 4 — Distributed Git *(from `book/ch05/` + material rescued from the GitHub chapter)*

**4.1 Distributed Workflows** (`distributed-workflows.asc`)
Centralized, integration-manager, dictator-and-lieutenants models; adds trunk-based
development and situates forge/PR collaboration within the taxonomy.
Status: **largely updated**.

**4.2 Contributing to a Project** (`contributing.asc`)
Commit guidelines; private small team; private managed team; public project over email
— email workflow retained but explicitly framed as the kernel/git.git niche;
`range-diff` referenced; the plaintext-IMAP-password example fixed.
Status: **largely updated**.

**4.3 Pull Requests and Forges** — *new section*
The fork → branch → PR → review → merge loop, forge-neutral; keeping a fork in sync
(rescues "fetch and push on different repositories" from the GitHub chapter); PR refs
(`refs/pull/*`, rescues `_pr_refs`); merge strategies (merge/squash/rebase); the `gh`
CLI. This is where the book's PR story now lives.
Status: **assembled + new** (core flow adapted from `ch06/2-contributing.asc` and
`ch06/3-maintaining.asc`, stripped of GitHub UI screenshots and rewritten forge-neutral
— rescued prose needs human rewriting, not just moving).

**4.4 Maintaining a Project** (`maintaining.asc`)
Topic branches, `apply`/`am`, checking out contributor branches, determining what's
introduced, integrating, tagging releases, shortlog; adds a PR-based maintenance
counterpart to the email-based flow.
Status: **largely updated**.

**4.5 Summary**
Status: **slightly updated** (the "next chapter: GitHub" handoff is rewritten).

---

## Chapter 5 — Git at Scale *(entirely new chapter)*

**5.0 Chapter intro**
Why big repos and big teams strain Git; a map of the scaling toolbox.
Status: **new**.

**5.1 Shallow Clones**
`--depth`, `--shallow-since`, deepening and unshallowing, CI patterns, the limitations
that make shallow the bluntest instrument.
Status: **new**.

**5.2 Partial Clone**
Blob/tree filters (`--filter=blob:none`), promisor remotes, lazy on-demand fetching,
`git backfill`; when partial beats shallow.
Status: **new**.

**5.3 Sparse Checkout**
`git sparse-checkout`, cone mode, the sparse index; working in a directory subset of a
huge repo.
Status: **new**.

**5.4 Scalar**
`scalar clone`/`register` as the batteries-included front door: what it enables
(partial clone, sparse checkout, background maintenance, FSMonitor).
Status: **new**.

**5.5 Keeping Large Repositories Fast**
`git maintenance` schedules, commit-graph, multi-pack-index, FSMonitor — the ops-level
performance story (Internals keeps the on-disk format details).
Status: **new**.

**5.6 Large Files**
Git LFS basics and when to prefer it vs. partial clone; keeping binaries out of
history.
Status: **new** (LFS is entirely absent from the current book).

**5.7 Monorepos**
Putting it together: monorepo trade-offs, layout and ownership practices, real-world
setups (e.g. Office/Windows on Scalar), forge support constraints.
Status: **new**.

**5.8 Summary**
Status: **new**.

---

## Chapter 6 — Git Toolkit *(from `book/ch07/`, renamed from "Git Tools")*

**6.1 Revision Selection** (`revision-selection.asc`)
Single revisions, short SHAs (64-char now), refs, ancestry, ranges, reflog.
Status: **not changed much** (mechanically heavy: hashes + 20 renames).

**6.2 Interactive Staging** (`interactive-staging.asc`)
`add -i`/`-p`, staging hunks.
Status: **not changed much**.

**6.3 Stashing and Cleaning** (`stashing-cleaning.asc`)
Stash/apply/pop/branch, `clean`; all `stash save` examples become `stash push`;
`stash show -u` noted.
Status: **slightly updated**.

**6.4 Signing Your Work** (`signing.asc`)
Adds SSH signing (`gpg.format=ssh`) as the mainstream path alongside GPG; modern key
recommendations; verifying signed commits/tags; how forges display verification.
Status: **largely updated** (significant new material).

**6.5 Searching** (`searching.asc`)
`git grep`, `log -S`/`-G`, line-log (`-L`).
Status: **not changed much**.

**6.6 Rewriting History** (`rewriting-history.asc`)
Amend, interactive rebase, autosquash (now non-interactive too); adds the `git history`
command (`reword`/`split`/`fixup`/`drop`) as the everyday tool with rebase -i as the
power tool; large-scale rewriting led by `filter-repo` with `filter-branch` demoted to
a deprecation note.
Status: **largely updated + new subsections**.

**6.7 Reset Demystified** (`reset.asc`)
The three trees, reset vs. checkout, path forms; cross-referenced with `restore`.
Status: **not changed much**.

**6.8 Advanced Merging** (`advanced-merging.asc`)
Conflict tooling, aborting/re-merging, `ours`/`theirs`, criss-cross situations; the
strategy is `ort` now; `git merge-tree` noted for tooling.
Status: **slightly updated**.

**6.9 Rerere** (`rerere.asc`)
Reuse recorded conflict resolutions.
Status: **not changed much**.

**6.10 Debugging with Git** (`debugging.asc`)
`blame`, bisect; adds `git bisect run` for automated hunts.
Status: **slightly updated**.

**6.11 Submodules** (`submodules.asc`)
Adding, updating, publishing, nesting submodules; "newer Git versions" framing
normalized to baseline.
Status: **slightly updated** (but the heaviest mechanical file in the book — 35 renames).

**6.12 Bundling** (`bundling.asc`)
`git bundle` for sneakernet transfer.
Status: **not changed much**.

**6.13 Replace** (`replace.asc`)
Reworked: grafts are *removed* in 3.0, so the section teaches `replace` (incl.
`--graft`) and the history-splicing use case without the grafts detour.
Status: **reworked**.

**6.14 Credential Storage** (`credentials.asc`)
Cache/store/osxkeychain and Git Credential Manager as the cross-platform standard;
custom credential helpers.
Status: **largely updated**.

**6.15 Summary**
Status: **slightly updated**.

*(Note: `ch07/subtree-merges.asc` exists on disk but is not included in the current
build; it stays out.)*

---

## Chapter 7 — Customizing Git *(from `book/ch08/`)*

**7.1 Git Configuration** (`config.asc`)
Config levels and the XDG path; the useful-knobs tour refreshed; adds `includeIf`
conditional config, `core.hooksPath`, `safe.directory`/`safe.bareRepository`,
`init.defaultBranch`, and the 3.0 extension knobs (`objectFormat`, `refStorage`).
Status: **largely updated**.

**7.2 Ignoring Files** — *promoted to its own section*
Pattern rules in depth, nested `.gitignore` files, global excludes
(`core.excludesFile`), debugging with `check-ignore`; Ch 2 keeps only the quick-start
basics and points here.
Status: **assembled** (moved from `ch02/recording-changes.asc` + `config.asc`, with a
small amount of new material).

**7.3 Git Attributes** (`attributes.asc`)
Diff/merge drivers, filters (smudge/clean), `export-ignore`; cross-references LFS in
Ch 5.
Status: **slightly updated**.

**7.4 Git Hooks** (`hooks.asc`)
Client and server hook catalog refreshed with post-2014 hooks; `core.hooksPath` for
shared hooks; a note on hook managers.
Status: **slightly updated**.

**7.5 An Example Git-Enforced Policy** (`policy.asc`)
The server-side enforcement capstone (update hook + client-side mirror).
Status: **not changed much** — *open decision: keep or cut; recommend keep* (it's the
payoff for 7.4 and unique material).

**7.6 Summary**
Status: **slightly updated**.

---

## Chapter 8 — Git and Agents *(entirely new chapter)*

**8.0 Chapter intro — Git in the age of agents**
Why AI coding agents change how repositories are used: many concurrent actors, high
commit volume, review becoming the bottleneck.
Status: **new**.

**8.1 Workflows for Agents**
Branch-per-agent, worktree-per-agent (builds on 3.5), sandboxed checkouts, commit/push
cadence, draft PRs as the agent hand-off unit.
Status: **new**.

**8.2 Tools and Interfaces**
How agents drive Git: the CLI as the universal interface, `gh`, MCP servers, forge
APIs and webhooks for automation.
Status: **new**.

**8.3 Libraries**
Programmatic Git for tool-builders: libgit2 (and its language bindings), JGit, go-git,
Dulwich, gitoxide — with SHA-256 support status per library.
Status: **assembled + new** (condensed reflow of dropped Appendix B, plus gitoxide and
the compatibility notes).

**8.4 Guiding Agents**
Repo-level agent instructions (AGENTS.md-style files), skills, commit-message and
branch-naming conventions for machine contributors, ignoring agent artifacts.
Status: **new**.

**8.5 Tips and Pitfalls**
Reviewing agent-authored history, attribution (co-author trailers), safety rails
(protected branches, force-push protection, `safe.*`), secrets hygiene.
Status: **new**.

**8.6 Summary**
Status: **new**.

---

## Chapter 9 — Git Servers *(from `book/ch04/`)*

**9.1 The Protocols** (`protocols.asc`)
Local, SSH, and smart HTTP (protocol v2) as the living protocols; dumb HTTP cut to a
historical note; `git://` demoted to legacy.
Status: **largely updated**.

**9.2 Getting Git on a Server** (`git-on-a-server.asc`)
Bare repositories, `--shared`, putting a repo on a box.
Status: **not changed much**.

**9.3 Generating Your SSH Public Key** (`generating-ssh-key.asc`)
Ed25519 as the recommended key type; DSA and `authorized_keys2`-era advice removed.
Status: **slightly updated** (mostly mechanical).

**9.4 Setting Up the Server** (`setting-up-server.asc`)
The `git` user, `authorized_keys`, `git-shell`; `xinetd`/sysvinit material dropped.
Status: **slightly updated**.

**9.5 Git Daemon** (`git-daemon.asc`)
Kept as a short treatment (systemd unit), clearly framed as niche.
Status: **trimmed**.

**9.6 Smart HTTP** (`smart-http.asc`)
`git-http-backend` behind a current Apache/nginx config; auth notes point at
tokens/OIDC rather than `.htpasswd` for anything public.
Status: **largely updated**.

**9.7 GitWeb** (`gitweb.asc`)
*Open decision:* cut to a sidebar mention or drop entirely; recommend cutting — modern
self-hosted forges cover this need.
Status: **cut** (recommended).

**9.8 Self-Hosted Forges** (reworked from `gitlab.asc`)
Running a full forge yourself: Gitea/Forgejo as the lightweight lead, GitLab as the
heavyweight; install sketch + day-one basics.
Status: **reworked** (was GitLab-only; screenshots regenerated).

**9.9 Hosted Options** (`hosted.asc`)
The hosting landscape — GitHub, GitLab.com, Codeberg, Sourcehut, Azure DevOps — and
how to choose between hosting and self-hosting.
Status: **reworked** (currently a GitHub/Bitbucket-era stub that defers to the dropped
GitHub chapter).

**9.10 Summary**
Status: **slightly updated**.

---

## Chapter 10 — Git Internals *(from `book/ch10/`)*

**10.1 Plumbing and Porcelain** (`plumbing-porcelain.asc`)
The `.git` directory tour, updated for what a 3.0 repo actually contains (reftable
dirs, no legacy `branches/`).
Status: **slightly updated**.

**10.2 Git Objects** (`objects.asc`)
Blobs, trees, commits, object storage — retaught with "object format" framing:
SHA-256 by default, SHA-1 interop; all hash-object walk-throughs regenerated.
Status: **largely updated**.

**10.3 Git References** (`refs.asc`)
Refs, HEAD, tags, remotes — plus a substantial new reftable subsection (the format,
and *why*: case-collisions, atomicity, performance) with loose/packed-refs reframed as
the legacy backend.
Status: **largely updated + new subsection**.

**10.4 Packfiles** (`packfiles.asc`)
Pack/idx formats and deltification; adds multi-pack-index and commit-graph notes
(format detail here; ops story lives in Ch 5).
Status: **largely updated**.

**10.5 The Refspec** (`refspec.asc`)
Fetch/push refspec syntax; the PR-refs cross-reference retargeted to 4.3.
Status: **slightly updated**.

**10.6 Transfer Protocols** (`transfer-protocols.asc`)
Rebuilt around protocol v2 (ref filtering, fetch negotiation); the dumb protocol
walk-through shrunk to a historical aside.
Status: **largely updated**.

**10.7 Maintenance and Data Recovery** (`maintenance.asc`)
`git maintenance` as the modern face of `gc`; reflog/`fsck` recovery; the
removing-a-huge-file example redone with `filter-repo`.
Status: **largely updated**.

**10.8 Environment Variables** (`environment.asc`)
The environment-variable survey.
Status: **not changed much**.

**10.9 Summary**
Status: **slightly updated**.

---

## Appendix A — Git Commands *(from `C-git-commands.asc`, the only appendix kept)*

Covers: the command-by-command index, category by category, retargeted at the new
chapter map. Adds entries for `switch`, `restore`, `worktree`, `sparse-checkout`,
`scalar`, `maintenance`, `range-diff`, `history`; notes the 3.0 removals
(`whatchanged`, `pack-redundant`) and deprecations (`filter-branch`); the "External
Systems" category shrinks or goes away with Ch 9 (SVN/P4/fast-import references lose
their backing chapter); broken xrefs into the dropped GitHub chapter
(`_pr_refs` ×2, `_email_notifications`) retarget to 4.3/4.4.
Status: **largely updated**.

---

## Dropped outright

| Dropped | Where its survivors go |
|---|---|
| Ch 6 GitHub (`ch06/`) | PR flow, fork syncing, PR refs → **4.3/4.4**; account setup, org management, Markdown, scripting/API → cut |
| Ch 9 Git and Other Systems (`ch09/`) | Cut entirely (SVN/Hg/P4 bridges, importers); Appendix A entries trimmed to match |
| Appendix A Git in Other Environments (`A/`) | Cut entirely (GUIs, IDEs, shell prompts) |
| Appendix B Embedding Git (`B/`) | Library survey reflows, condensed, into **8.3** |
| `introduction.asc` | *Open decision:* fold into the Preface or Ch 1 opener; recommend folding into the Preface |

## Open decisions (flagged inline above)

1. `policy.asc` (7.5) — keep (recommended) or cut.
2. GitWeb (9.7) — cut to sidebar (recommended) or keep trimmed.
3. `introduction.asc` — fold into Preface (recommended) or keep as a standalone opener.
4. Large Files (5.6) — lives in Git at Scale (recommended) vs. Git Servers.
5. Performance split — ops story in 5.5, format detail in 10.4/10.7 (recommended), or consolidate in one place.

## Mechanical follow-through (no prose)

- New include map in `progit.asc`; new wrapper files for Ch 5 and Ch 8; directory
  renames if chapter dirs are renumbered.
- Fix inbound xrefs: `ch10/refspec.asc` and `C-git-commands.asc` → dropped GitHub
  anchors; `ch05/contributing.asc` and `ch04/hosted.asc` → `ch06-github`; Ch 2's
  `_ignoring` anchor gains a new home in 7.2.
- Chapter-order prose ("next chapter…" in the Ch 4 and Ch 5 wrappers) needs human
  sentence edits for the new sequence.
- Diagram regeneration in `diagram-source/` for Ch 3 (main-named branches) and new
  diagrams for 3.5, Ch 5, and Ch 8.
