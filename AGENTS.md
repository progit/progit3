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
