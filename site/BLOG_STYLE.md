# Blog writing style guide

Blog posts on the Pro Git site are written in the same voice as the book.
This guide describes that voice, derived from the book's own text, so that posts read
like they were written by the book's authors — because stylistically, they are.

**Anyone (human or agent) writing or editing a post in `src/content/blog/` must read
this guide first and follow it.**

Every trait below is illustrated with a real sentence from the book.

## Voice and person

**Address the reader as "you"; the authors are "we".**
The book talks directly to one reader and speaks as its authors, never in an
impersonal third person.

> You're about to spend several hours of your life reading about Git.
> Let's take a minute to explain what we have in store for you.
> — *Introduction*

**Use "Let's" to start a walkthrough or a worked example.**

> To visualize this, let's assume that you have a directory containing three files,
> and you stage them all and commit.
> — *Branches in a Nutshell*

**Use contractions.**
"You'll", "it's", "don't", "can't", "we're". The book is conversational, not formal.

## Openers

**Get to the point in the first sentence.**
Posts open with the fact, then explain. No throat-clearing, no scene-setting.

> So, what is Git in a nutshell?
> — *What is Git?*

**Rhetorical questions are welcome — but answer them immediately.**

> What happens when you create a new branch?
> Well, doing so creates a new pointer for you to move around.
> — *Creating a New Branch*

## Tone

**Plain, concrete, and direct.** Explain *why* before *how*.

> This is an important section to absorb, because if you understand what Git is and
> the fundamentals of how it works, then using Git effectively will probably be much
> easier for you.
> — *What is Git?*

**No hype and no marketing language.**
Never "excited to announce", "seamless", "robust", "leverage", "delve", "game-changing".
When the book praises something, it says specifically what is good about it.

**Dry, understated humor — sparingly.** At most one flourish per post, and only when
it lands naturally.

> If the book spontaneously combusts at this point, you should already be pretty
> useful wielding Git in the time it takes you to go pick up another copy.
> — *Introduction*

**Tell the reader when something matters.**

> Pay attention now — here is the main thing to remember about Git if you want the
> rest of your learning process to go smoothly.
> — *The Three States*

## Structure

**Short paragraphs, one point each.** One to four sentences is typical.

**Signpost what's coming, and end with a bridge.**
The book constantly tells you where you are going next and closes sections by pointing
forward. Posts should end the same way: where to follow along, what happens next, or
where to pitch in.

> Let's get started.
> — *Introduction*

**Lists are for enumerable facts; numbered lists are for step sequences.**
The book uses bullets for parallel facts (the three states) and numbers for workflows
(the basic Git workflow). Everything else is prose.

## Sentences and punctuation

**Mostly short and medium declarative sentences.** Vary the rhythm; don't stack three
long sentences in a row.

**Em dashes for asides**, surrounded by spaces (the book's ` -- ` renders as an
em dash; in MDX write ` — ` directly).

> Most operations in Git need only local files and resources to operate — generally
> no information is needed from another computer on your network.
> — *Nearly Every Operation Is Local*

**Parentheses for quick side remarks.**

> …you can commit happily (to your _local_ copy, remember?) until you get to a
> network connection to upload.
> — *Nearly Every Operation Is Local*

**No exclamation points. No emoji.**

## Formatting conventions

- `monospace` for commands, branch names, file names, config keys: `git branch`,
  `main`, `AGENTS.md`.
- _Italics_ for a new term at first use, the way the book introduces _snapshots_
  and _blobs_. Also for book titles: _Pro Git_.
- **Bold** sparingly, for the one key phrase in a paragraph — the book bolds
  *stream of snapshots*, not whole sentences.
- Headings only when a post is long enough to need them; short posts are just prose.

## Source formatting

**One sentence per line.**
The book's AsciiDoc sources put each sentence on its own line (semantic line breaks),
which keeps diffs reviewable. Do the same in post MDX; Markdown joins the lines when
rendering.

## Frontmatter

- `description` is one plain sentence, in the same voice.
- Agent-written posts must set `automated: true` (see `README.md`); this renders the
  "written by AI" banner and is non-negotiable.
