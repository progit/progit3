# `master` → `main` Change Inventory

_Generated from the current book source. Every word-boundary occurrence of `master` in `book/**/*.asc`, classified by required action. Total: **615** occurrences in 50 files._

## Summary by action

| Action | Count | What to do |
|--------|------:|------------|
| **Rename** (example repos) | 602 | Mechanical `master`→`main` in our own example output/commands/diagrams. |
| **Verify external URL** | 11 | Points at a *third-party* repo path. Confirm that project's current default branch; many have renamed or archived. Do **not** blind-rename. |
| **Intentional naming text** | 2 | Prose that deliberately lists `master/main/mainline`. Keep the concept; revisit wording for a main-default world. |
| **English word** (false positive) | 0 | None found. |
| **TOTAL** | **615** | |

> The 602 renames are not fully mechanical: renaming the branch in example *output* also requires regenerating any **diagram** (`diagram-source/`) and **screenshot** that shows the branch, and keeping ref names consistent across a chapter's running example. Treat each file as one coordinated edit, not N independent substitutions.

## Renames by chapter

| Chapter | Renames |
|---------|--------:|
| 02-git-basics | 58 |
| 03-git-branching | 101 |
| 04-git-server | 3 |
| 05-distributed-git | 92 |
| 06-github | 29 |
| 07-git-tools | 147 |
| 08-customizing-git | 12 |
| 09-git-and-other-scms | 84 |
| 10-git-internals | 70 |
| B-embedding-git | 5 |
| introduction.asc | 1 |

## Renames by file

| File | Renames |
|------|--------:|
| `book/05-distributed-git/sections/contributing.asc` | 48 |
| `book/05-distributed-git/sections/maintaining.asc` | 39 |
| `book/09-git-and-other-scms/sections/client-p4.asc` | 37 |
| `book/02-git-basics/sections/recording-changes.asc` | 35 |
| `book/07-git-tools/sections/submodules.asc` | 35 |
| `book/06-github/sections/2-contributing.asc` | 26 |
| `book/03-git-branching/sections/rebasing.asc` | 24 |
| `book/09-git-and-other-scms/sections/client-hg.asc` | 23 |
| `book/03-git-branching/sections/basic-branching-and-merging.asc` | 22 |
| `book/07-git-tools/sections/revision-selection.asc` | 20 |
| `book/10-git-internals/sections/refspec.asc` | 20 |
| `book/03-git-branching/sections/branch-management.asc` | 18 |
| `book/03-git-branching/sections/remote-branches.asc` | 18 |
| `book/07-git-tools/sections/advanced-merging.asc` | 17 |
| `book/07-git-tools/sections/replace.asc` | 17 |
| `book/02-git-basics/sections/remotes.asc` | 15 |
| `book/03-git-branching/sections/nutshell.asc` | 15 |
| `book/10-git-internals/sections/transfer-protocols.asc` | 15 |
| `book/07-git-tools/sections/bundling.asc` | 14 |
| `book/07-git-tools/sections/stashing-cleaning.asc` | 14 |
| `book/07-git-tools/sections/subtree-merges.asc` | 13 |
| `book/10-git-internals/sections/refs.asc` | 10 |
| `book/08-customizing-git/sections/policy.asc` | 8 |
| `book/09-git-and-other-scms/sections/client-svn.asc` | 8 |
| `book/10-git-internals/sections/maintenance.asc` | 8 |
| `book/02-git-basics/sections/undoing.asc` | 7 |
| `book/07-git-tools/sections/reset.asc` | 7 |
| `book/09-git-and-other-scms/sections/import-custom.asc` | 6 |
| `book/09-git-and-other-scms/sections/import-hg.asc` | 6 |
| `book/10-git-internals/sections/environment.asc` | 6 |
| `book/10-git-internals/sections/objects.asc` | 6 |
| `book/05-distributed-git/sections/distributed-workflows.asc` | 5 |
| `book/07-git-tools/sections/rerere.asc` | 5 |
| `book/10-git-internals/sections/packfiles.asc` | 5 |
| `book/B-embedding-git/sections/jgit.asc` | 5 |
| `book/03-git-branching/sections/workflows.asc` | 4 |
| `book/08-customizing-git/sections/config.asc` | 4 |
| `book/06-github/sections/3-maintaining.asc` | 3 |
| `book/07-git-tools/sections/rewriting-history.asc` | 3 |
| `book/04-git-server/sections/setting-up-server.asc` | 2 |
| `book/09-git-and-other-scms/sections/import-p4.asc` | 2 |
| `book/09-git-and-other-scms/sections/import-svn.asc` | 2 |
| `book/02-git-basics/sections/viewing-history.asc` | 1 |
| `book/04-git-server/sections/gitlab.asc` | 1 |
| `book/07-git-tools/sections/interactive-staging.asc` | 1 |
| `book/07-git-tools/sections/signing.asc` | 1 |
| `book/introduction.asc` | 1 |

## Special cases — do NOT blind-rename

### External URLs (verify target repo's current default branch)

Each points at a third-party repository path pinned to `master`. Check whether that project renamed to `main` (update the URL) or was archived/moved (replace or drop the reference).

| File:Line | URL fragment |
|-----------|--------------|
| `book/B-embedding-git/sections/go-git.asc:8` | https://github.com/go-git/go-git/blob/master |
| `book/B-embedding-git/sections/go-git.asc:54` | https://github.com/go-git/go-git/tree/master |
| `book/B-embedding-git/sections/go-git.asc:59` | https://github.com/go-git/go-git/blob/master |
| `book/B-embedding-git/sections/go-git.asc:83` | https://github.com/go-git/go-git/tree/master |
| `book/09-git-and-other-scms/sections/client-hg.asc:21` | https://raw.githubusercontent.com/felipec/git-remote-hg/master |
| `book/A-git-in-other-environments/sections/zsh.asc:45` | https://github.com/git/git/blob/master |
| `book/A-git-in-other-environments/sections/powershell.asc:85` | https://github.com/dahlbyk/posh-git/blob/master |
| `book/A-git-in-other-environments/sections/powershell.asc:86` | https://github.com/dahlbyk/posh-git/blob/master |
| `book/10-git-internals/sections/packfiles.asc:27` | https://raw.githubusercontent.com/mojombo/grit/master |
| `book/05-distributed-git/sections/maintaining.asc:367` | https://github.com/git/git/blob/master |
| `book/04-git-server/sections/gitlab.asc:22` | https://gitlab.com/gitlab-org/gitlab-foss/-/blob/master |

_Notes for the pass: `git/git` itself still uses `master`; `go-git`, `posh-git`, and `git-remote-hg` should be checked individually; `mojombo/grit` is archived (dead example — replace)._

### Intentional naming discussion (keep concept, revisit wording)

- `book/03-git-branching/sections/branch-management.asc:82` — Do not rename a branch like master/main/mainline without having read the section <<_changing_master>>.
- `book/03-git-branching/sections/branch-management.asc:131` — Changing the name of a branch like master/main/mainline/default will break the integrations, services, helper utilities and build/release scripts that your repository uses.

## Full line-level checklist (example-rename bucket)

Grouped by file, ascending line number. Check off as each file's running example is migrated (and its diagrams/screenshots regenerated).


### `book/02-git-basics/sections/recording-changes.asc`

- [ ] L28: `On branch master`
- [ ] L29: `Your branch is up-to-date with 'origin/master'.`
- [ ] L36: `For now, that branch is always `master`, which is the default; you won't worry about it here.`
- [ ] L41: `GitHub changed the default branch name from `master` to `main` in mid-2020, and other Git hosts followed suit.`
- [ ] L42: `So you may find that the default branch name in some newly created repositories is `main` and not `master`.`
- [ ] L45: `However, Git itself still uses `master` as the default, so we will use it throughout the book.`
- [ ] L55: `On branch master`
- [ ] L56: `Your branch is up-to-date with 'origin/master'.`
- [ ] L86: `On branch master`
- [ ] L87: `Your branch is up-to-date with 'origin/master'.`
- [ ] L108: `On branch master`
- [ ] L109: `Your branch is up-to-date with 'origin/master'.`
- [ ] L133: `On branch master`
- [ ] L134: `Your branch is up-to-date with 'origin/master'.`
- [ ] L152: `On branch master`
- [ ] L153: `Your branch is up-to-date with 'origin/master'.`
- [ ] L179: `On branch master`
- [ ] L180: `Your branch is up-to-date with 'origin/master'.`
- [ ] L293: `On branch master`
- [ ] L294: `Your branch is up-to-date with 'origin/master'.`
- [ ] L357: `On branch master`
- [ ] L358: `Your branch is up-to-date with 'origin/master'.`
- [ ] L445: `# On branch master`
- [ ] L446: `# Your branch is up-to-date with 'origin/master'.`
- [ ] L474: `[master 463dc4f] Story 182: fix benchmarks for speed`
- [ ] L480: `You can see that the commit has given you some output about itself: which branch you committed to (`master`), what SHA-1 checksum the com...`
- [ ] L496: `On branch master`
- [ ] L497: `Your branch is up-to-date with 'origin/master'.`
- [ ] L506: `[master 83e38c7] Add new benchmarks`
- [ ] L527: `On branch master`
- [ ] L528: `Your branch is up-to-date with 'origin/master'.`
- [ ] L545: `On branch master`
- [ ] L546: `Your branch is up-to-date with 'origin/master'.`
- [ ] L610: `On branch master`
- [ ] L611: `Your branch is up-to-date with 'origin/master'.`

### `book/02-git-basics/sections/remotes.asc`

- [ ] L101: `* [new branch]      master     -> pb/master`
- [ ] L105: `Paul's `master` branch is now accessible locally as `pb/master` -- you can merge it into one of your branches, or you can check out a loc...`
- [ ] L127: `This may be an easier or more comfortable workflow for you; and by default, the `git clone` command automatically sets up your local `mas...`
- [ ] L147: `If you want to push your `master` branch to your `origin` server (again, cloning generally sets up both of those names for you automatica...`
- [ ] L151: `$ git push origin master`
- [ ] L171: `HEAD branch: master`
- [ ] L173: `master                               tracked`
- [ ] L176: `master merges with remote master`
- [ ] L178: `master pushes to master (up to date)`
- [ ] L182: `The command helpfully tells you that if you're on the `master` branch and you run `git pull`, it will automatically merge the remote's `m...`
- [ ] L195: `HEAD branch: master`
- [ ] L197: `master                           tracked`
- [ ] L205: `master     merges with remote master`
- [ ] L209: `master                         pushes to master                         (up to date)`
- [ ] L229: `What used to be referenced at `pb/master` is now at `paul/master`.`

### `book/02-git-basics/sections/undoing.asc`

- [ ] L62: `On branch master`
- [ ] L79: `On branch master`
- [ ] L102: `We'll go into much more detail about what `reset` does and how to master it to do really interesting things in <<ch07-git-tools#_git_rese...`
- [ ] L127: `On branch master`
- [ ] L171: `On branch master`
- [ ] L186: `On branch master`
- [ ] L223: `On branch master`

### `book/02-git-basics/sections/viewing-history.asc`

- [ ] L194: `*  5e3ee11 Merge branch 'master' of https://github.com/dustin/grit.git`

### `book/03-git-branching/sections/basic-branching-and-merging.asc`

- [ ] L22: `First, let's say you're working on your project and have a couple of commits already on the `master` branch.`
- [ ] L61: `All you have to do is switch back to your `master` branch.`
- [ ] L66: `For now, let's assume you've committed all your changes, so you can switch back to your `master` branch:`
- [ ] L70: `$ git checkout master`
- [ ] L71: `Switched to branch 'master'`
- [ ] L91: `.Hotfix branch based on `master``
- [ ] L92: `image::images/basic-branching-4.png[Hotfix branch based on `master`]`
- [ ] L94: `You can run your tests, make sure the hotfix is what you want, and finally merge the `hotfix` branch back into your `master` branch to de...`
- [ ] L99: `$ git checkout master`
- [ ] L111: `Your change is now in the snapshot of the commit pointed to by the `master` branch, and you can deploy the fix.`
- [ ] L113: `.`master` is fast-forwarded to `hotfix``
- [ ] L114: `image::images/basic-branching-5.png[`master` is fast-forwarded to `hotfix`]`
- [ ] L117: `However, first you'll delete the `hotfix` branch, because you no longer need it -- the `master` branch points at the same place.`
- [ ] L142: `If you need to pull it in, you can merge your `master` branch into your `iss53` branch by running `git merge master`, or you can wait to ...`
- [ ] L148: `Suppose you've decided that your issue #53 work is complete and ready to be merged into your `master` branch.`
- [ ] L149: `In order to do that, you'll merge your `iss53` branch into `master`, much like you merged your `hotfix` branch earlier.`
- [ ] L154: `$ git checkout master`
- [ ] L155: `Switched to branch 'master'`
- [ ] L207: `On branch master`
- [ ] L234: `This means the version in `HEAD` (your `master` branch, because that was what you had checked out when you ran your merge command) is the...`
- [ ] L283: `On branch master`
- [ ] L310: `# On branch master`

### `book/03-git-branching/sections/branch-management.asc`

- [ ] L14: `* master`
- [ ] L18: `Notice the `*` character that prefixes the `master` branch: it indicates the branch that you currently have checked out (i.e., the branch...`
- [ ] L19: `This means that if you commit at this point, the `master` branch will be moved forward with your new work.`
- [ ] L26: `* master  7a98805 Merge branch 'iss53'`
- [ ] L37: `* master`
- [ ] L67: `You can always provide an additional argument to ask about the merge state with respect to some other branch without checking that other ...`
- [ ] L71: `$ git branch --no-merged master`
- [ ] L127: `===== Changing the master branch name`
- [ ] L136: `Rename your local `master` branch into `main` with the following command:`
- [ ] L140: `$ git branch --move master main`
- [ ] L143: `There's no local `master` branch anymore, because it's renamed to the `main` branch.`
- [ ] L159: `remotes/origin/HEAD -> origin/master`
- [ ] L161: `remotes/origin/master`
- [ ] L164: `Your local `master` branch is gone, as it's replaced with the `main` branch.`
- [ ] L166: `However, the old `master` branch is still present on the remote.`
- [ ] L167: `Other collaborators will continue to use the `master` branch as the base of their work, until you make some further changes.`
- [ ] L178: `After you've done all these tasks, and are certain the `main` branch performs just as the `master` branch, you can delete the `master` br...`
- [ ] L182: `$ git push origin --delete master`

### `book/03-git-branching/sections/nutshell.asc`

- [ ] L34: `The default branch name in Git is `master`.`
- [ ] L35: `As you start making commits, you're given a `master` branch that points to the last commit you made.`
- [ ] L36: `Every time you commit, the `master` branch pointer moves forward automatically.`
- [ ] L40: `The "`master`" branch in Git is not a special branch.(((master)))`
- [ ] L71: `In this case, you're still on `master`.`
- [ ] L75: `image::images/head-to-master.png[HEAD pointing to a branch]`
- [ ] L83: `f30ab (HEAD -> master, testing) Add feature #32 - ability to add new formats to the central interface`
- [ ] L88: `You can see the `master` and `testing` branches that are right there next to the `f30ab` commit.`
- [ ] L119: `This is interesting, because now your `testing` branch has moved forward, but your `master` branch still points to the commit you were on...`
- [ ] L120: `Let's switch back to the `master` branch:`
- [ ] L124: `$ git checkout master`
- [ ] L140: `image::images/checkout-master.png[HEAD moves when you checkout]`
- [ ] L143: `It moved the HEAD pointer back to point to the `master` branch, and it reverted the files in your working directory back to the snapshot ...`
- [ ] L170: `image::images/advance-master.png[Divergent history]`
- [ ] L178: `* c2b9e (HEAD, master) Make other changes`

### `book/03-git-branching/sections/rebasing.asc`

- [ ] L26: `For this example, you would check out the `experiment` branch, and then rebase it onto the `master` branch as follows:`
- [ ] L31: `$ git rebase master`
- [ ] L41: `At this point, you can go back to the `master` branch and do a fast-forward merge.`
- [ ] L45: `$ git checkout master`
- [ ] L49: `.Fast-forwarding the `master` branch`
- [ ] L50: `image::images/basic-rebase-4.png[Fast-forwarding the `master` branch]`
- [ ] L57: `In this case, you'd do your work in a branch and then rebase your work onto `origin/master` when you were ready to submit your patches to...`
- [ ] L76: `You can take the changes on `client` that aren't on `server` (`C8` and `C9`) and replay them on your `master` branch by using the `--onto...`
- [ ] L80: `$ git rebase --onto master server client`
- [ ] L83: `This basically says, "`Take the `client` branch, figure out the patches since it diverged from the `server` branch, and replay these patc...`
- [ ] L89: `Now you can fast-forward your `master` branch (see <<rbdiag_g>>):`
- [ ] L93: `$ git checkout master`
- [ ] L98: `.Fast-forwarding your `master` branch to include the `client` branch changes`
- [ ] L99: `image::images/interesting-rebase-3.png[Fast-forwarding your `master` branch to include the `client` branch changes]`
- [ ] L102: `You can rebase the `server` branch onto the `master` branch without having to check it out first by running `git rebase <basebranch> <top...`
- [ ] L106: `$ git rebase master server`
- [ ] L109: `This replays your `server` work on top of your `master` work, as shown in <<rbdiag_h>>.`
- [ ] L112: `.Rebasing your `server` branch on top of your `master` branch`
- [ ] L113: `image::images/interesting-rebase-4.png[Rebasing your `server` branch on top of your `master` branch]`
- [ ] L115: `Then, you can fast-forward the base branch (`master`):`
- [ ] L119: `$ git checkout master`
- [ ] L191: `For instance, in the previous scenario, if instead of doing a merge when we're at <<_pre_merge_rebase_work>> we run `git rebase teamone/m...`
- [ ] L196: `* Apply those commits to the top of `teamone/master``
- [ ] L208: `Or you could do it manually with a `git fetch` followed by a `git rebase teamone/master` in this case.`

### `book/03-git-branching/sections/remote-branches.asc`

- [ ] L14: `For instance, if you wanted to see what the `master` branch on your `origin` remote looked like as of the last time you communicated with...`
- [ ] L19: `If you clone from this, Git's `clone` command automatically names it `origin` for you, pulls down all its data, creates a pointer to wher...`
- [ ] L20: `Git also gives you your own local `master` branch starting at the same place as origin's `master` branch, so you have something to work f...`
- [ ] L25: `Just like the branch name "`master`" does not have any special meaning in Git, neither does "`origin`".`
- [ ] L26: `While "`master`" is the default name for a starting branch when you run `git init` which is the only reason it's widely used, "`origin`" ...`
- [ ] L27: `If you run `git clone -o booyah` instead, then you will have `booyah/master` as your default remote branch.(((origin)))`
- [ ] L33: `If you do some work on your local `master` branch, and, in the meantime, someone else pushes to `git.ourcompany.com` and updates its `mas...`
- [ ] L34: `Also, as long as you stay out of contact with your `origin` server, your `origin/master` pointer doesn't move.`
- [ ] L40: `This command looks up which server "`origin`" is (in this case, it's `git.ourcompany.com`), fetches any data from it that you don't yet h...`
- [ ] L54: `Because that server has a subset of the data your `origin` server has right now, Git fetches no data but sets a remote-tracking branch ca...`
- [ ] L56: `.Remote-tracking branch for `teamone/master``
- [ ] L57: `image::images/remote-branches-5.png[Remote-tracking branch for `teamone/master`]`
- [ ] L137: `When you clone a repository, it generally automatically creates a `master` branch that tracks `origin/master`.`
- [ ] L138: `However, you can set up other tracking branches if you wish -- ones that track branches on other remotes, or don't track the `master` bra...`
- [ ] L182: `So if you're on the `master` branch and it's tracking `origin/master`, you can say something like `git merge @{u}` instead of `git merge ...`
- [ ] L192: `master    1ae2a45 [origin/master] Deploy index fix`
- [ ] L198: `We can also see that our `master` branch is tracking `origin/master` and is up to date.`
- [ ] L224: `Suppose you're done with a remote branch -- say you and your collaborators are finished with a feature and have merged it into your remot...`

### `book/03-git-branching/sections/workflows.asc`

- [ ] L12: `Many Git developers have a workflow that embraces this approach, such as having only code that is entirely stable in their `master` branc...`
- [ ] L13: `They have another parallel branch named `develop` or `next` that they work from or use to test stability -- it isn't necessarily always s...`
- [ ] L29: `Some larger projects also have a `proposed` or `pu` (proposed updates) branch that has integrated branches that may not be ready to go in...`
- [ ] L47: `Consider an example of doing some work (on `master`), branching off for an issue (`iss91`), working on it for a bit, branching off the se...`

### `book/04-git-server/sections/gitlab.asc`

- [ ] L121: `Users with direct access can simply create a branch, push commits to it, and open a merge request from their branch back into `master` or...`

### `book/04-git-server/sections/setting-up-server.asc`

- [ ] L72: `$ git push origin master`
- [ ] L83: `$ git push origin master`

### `book/05-distributed-git/sections/contributing.asc`

- [ ] L124: `[master 738ee87] Remove invalid default value`
- [ ] L139: `[master fbff5bc] Add reset task`
- [ ] L148: `$ git push origin master`
- [ ] L151: `1edee6b..fbff5bc  master -> master`
- [ ] L164: `$ git push origin master`
- [ ] L166: `! [rejected]        master -> master (non-fast forward)`
- [ ] L182: `+ 049d078...fbff5bc master     -> origin/master`
- [ ] L194: `$ git merge origin/master`
- [ ] L202: `.John's repository after merging `origin/master``
- [ ] L203: `image::images/small-team-2.png[John's repository after merging `origin/master`]`
- [ ] L209: `$ git push origin master`
- [ ] L212: `fbff5bc..72bbc59  master -> master`
- [ ] L234: `fbff5bc..72bbc59  master     -> origin/master`
- [ ] L248: `$ git log --no-merges issue54..origin/master`
- [ ] L256: `The `issue54..origin/master` syntax is a log filter that asks Git to display only those commits that are on the latter branch (in this ca...`
- [ ] L260: `If she merges `origin/master`, that is the single commit that will modify her local work.`
- [ ] L262: `Now, Jessica can merge her topic work into her `master` branch, merge John's work (`origin/master`) into her `master` branch, and then pu...`
- [ ] L264: `First (having committed all of the work on her `issue54` topic branch), Jessica switches back to her `master` branch in preparation for i...`
- [ ] L268: `$ git checkout master`
- [ ] L269: `Switched to branch 'master'`
- [ ] L270: `Your branch is behind 'origin/master' by 2 commits, and can be fast-forwarded.`
- [ ] L273: `Jessica can merge either `origin/master` or `issue54` first -- they're both upstream, so the order doesn't matter.`
- [ ] L288: `Jessica now completes the local merging process by merging John's earlier fetched work that is sitting in the `origin/master` branch:`
- [ ] L292: `$ git merge origin/master`
- [ ] L304: `Now `origin/master` is reachable from Jessica's `master` branch, so she should be able to successfully push (assuming John hasn't pushed ...`
- [ ] L308: `$ git push origin master`
- [ ] L311: `72bbc59..8059c15  master -> master`
- [ ] L320: `You work for a while (generally in a topic branch), and merge that work into your `master` branch when it's ready to be integrated.`
- [ ] L321: `When you want to share that work, you fetch and merge your `master` from `origin/master` if it has changed, and finally push to the `mast...`
- [ ] L334: `In this case, the company is using a type of integration-manager workflow where the work of the individual groups is integrated only by c...`
- [ ] L353: `Jessica doesn't have push access to the `master` branch -- only the integrators do -- so she has to push to another branch in order to co...`
- [ ] L365: `To begin, she starts a new feature branch, basing it off the server's `master` branch:`
- [ ] L371: `$ git checkout -b featureB origin/master`
- [ ] L537: `It's easiest to push the topic branch you're working on to your forked repository, rather than merging that work into your `master` branc...`
- [ ] L538: `The reason is that if your work isn't accepted or is cherry-picked, you don't have to rewind your `master` branch (the Git `cherry-pick` ...`
- [ ] L557: `$ git request-pull origin/master myfork`
- [ ] L576: `On a project for which you're not the maintainer, it's generally easier to have a branch like `master` always track `origin/master` and t...`
- [ ] L578: `For example, if you want to submit a second topic of work to the project, don't continue working on the topic branch you just pushed up -...`
- [ ] L582: `$ git checkout -b featureB origin/master`
- [ ] L586: `$ git request-pull origin/master myfork`
- [ ] L597: `In this case, you can try to rebase that branch on top of `origin/master`, resolve the conflicts for the maintainer, and then resubmit yo...`
- [ ] L602: `$ git rebase origin/master`
- [ ] L616: `You'll also take this opportunity to move the work to be based off the project's current `master` branch.`
- [ ] L617: `You start a new branch based off the current `origin/master` branch, squash the `featureB` changes there, resolve any conflicts, make the...`
- [ ] L622: `$ git checkout -b featureBv2 origin/master`
- [ ] L665: `$ git format-patch -M origin/master`
- [ ] L695: `def log(treeish = 'master')`
- [ ] L700: `def ls_tree(treeish = 'master')`

### `book/05-distributed-git/sections/distributed-workflows.asc`

- [ ] L73: `1.  Regular developers work on their topic branch and rebase their work on top of `master`.`
- [ ] L74: `The `master` branch is that of the reference repository to which the dictator pushes.`
- [ ] L75: `2.  Lieutenants merge the developers' topic branches into their `master` branch.`
- [ ] L76: `3.  The dictator merges the lieutenants' `master` branches into the dictator's `master` branch.`
- [ ] L77: `4.  Finally, the dictator pushes that `master` branch to the reference repository so the other developers can rebase on it.`

### `book/05-distributed-git/sections/maintaining.asc`

- [ ] L15: `As you'll remember, you can create the branch based off your `master` branch like this:`
- [ ] L19: `$ git branch sc/ruby_client master`
- [ ] L26: `$ git checkout -b sc/ruby_client master`
- [ ] L228: `It's often helpful to get a review of all the commits that are in this branch but that aren't in your `master` branch.`
- [ ] L229: `You can exclude commits in the `master` branch by adding the `--not` option before the branch name.`
- [ ] L230: `This does the same thing as the `master..contrib` format that we used earlier.`
- [ ] L235: `$ git log contrib --not master`
- [ ] L256: `$ git diff master`
- [ ] L260: `If your `master` branch has moved forward since you created the topic branch from it, then you'll get seemingly strange results.`
- [ ] L261: `This happens because Git directly compares the snapshots of the last commit of the topic branch you're on and the snapshot of the last co...`
- [ ] L262: `For example, if you've added a line in a file on the `master` branch, a direct comparison of the snapshots will look like the topic branc...`
- [ ] L264: `If `master` is a direct ancestor of your topic branch, this isn't a problem; but if the two histories have diverged, the diff will look l...`
- [ ] L266: `What you really want to see are the changes added to the topic branch -- the work you'll introduce if you merge this branch with `master`.`
- [ ] L267: `You do that by having Git compare the last commit on your topic branch with the first common ancestor it has with the `master` branch.`
- [ ] L273: `$ git merge-base contrib master`
- [ ] L282: `$ git diff $(git merge-base contrib master)`
- [ ] L290: `$ git diff master...contrib`
- [ ] L293: `This command shows you only the work your current topic branch has introduced since its common ancestor with `master`.`
- [ ] L306: `One basic workflow is to simply merge all that work directly into your `master` branch.`
- [ ] L307: `In this scenario, you have a `master` branch that contains basically stable code.`
- [ ] L308: `When you have work in a topic branch that you think you've completed, or work that someone else has contributed and you've verified, you ...`
- [ ] L323: `In this scenario, you have two long-running branches, `master` and `develop`, in which you determine that `master` is updated only when a...`
- [ ] L325: `Each time you have a new topic branch to merge in (<<merwf_c>>), you merge it into `develop` (<<merwf_d>>); then, when you tag a release,...`
- [ ] L339: `This way, when people clone your project's repository, they can either check out `master` to build the latest stable version and keep up ...`
- [ ] L341: `Then, when the codebase on that branch is stable and passes tests, you merge it into a `develop` branch; and when that has proven itself ...`
- [ ] L346: `The Git project has four long-running branches: `master`, `next`, and `seen` (formerly 'pu' -- proposed updates) for new work, and `maint...`
- [ ] L356: `When it's determined that they're totally stable, the topics are re-merged into `master`.`
- [ ] L357: `The `next` and `seen` branches are then rebuilt from the `master`.`
- [ ] L358: `This means `master` almost always moves forward, `next` is rebased occasionally, and `seen` is rebased even more often:`
- [ ] L363: `When a topic branch has finally been merged into `master`, it's removed from the repository.`
- [ ] L373: `Other maintainers prefer to rebase or cherry-pick contributed work on top of their `master` branch, rather than merging it in, to keep a ...`
- [ ] L374: `When you have work in a topic branch and have determined that you want to integrate it, you move to that branch and run the rebase comman...`
- [ ] L375: `If that works well, you can fast-forward your `master` branch, and you'll end up with a linear project history.`
- [ ] L387: `If you want to pull commit `e43a6` into your `master` branch, you can run:`
- [ ] L393: `[master]: created a0a41a9: "More friendly message when locking the index fails."`
- [ ] L493: `$ git describe master`
- [ ] L515: `$ git archive master --prefix='project/' \| gzip > `git describe master`.tar.gz`
- [ ] L525: `$ git archive master --prefix='project/' --format=zip > `git describe master`.zip`
- [ ] L540: `$ git shortlog --no-merges master --not v1.0.1`

### `book/06-github/sections/2-contributing.asc`

- [ ] L39: `2. Create a topic branch from `master`.`
- [ ] L45: `8. Sync the updated `master` back to your fork.`
- [ ] L134: `We also see a list of the commits in our topic branch that are "`ahead`" of the `master` branch (in this case, just the one) and a unifie...`
- [ ] L185: `In `git diff` terms, it basically automatically shows you `git diff master...<branch>` for the branch this Pull Request is based on.`
- [ ] L193: `If you merge this branch into the `master` branch and push it to GitHub, the Pull Request will automatically be closed.`
- [ ] L202: `If you're working on a feature with someone and you both have write access to the project, you can push a topic branch to the repository ...`
- [ ] L236: `You can either rebase your branch on top of whatever the target branch is (normally the `master` branch of the repository you forked), or...`
- [ ] L256: `* [new branch]      master     -> upstream/master`
- [ ] L258: `$ git merge upstream/master <3>`
- [ ] L266: `[slow-blink 3c8d735] Merge remote-tracking branch 'upstream/master' \`
- [ ] L496: `This branch is 5 commits behind progit:master.`
- [ ] L503: `For example, if you forked from `https://github.com/progit/progit2.git`, you can keep your `master` branch up-to-date like this:`
- [ ] L507: `$ git checkout master <1>`
- [ ] L509: `$ git push origin master <3>`
- [ ] L512: `<1> If you were on another branch, return to `master`.`
- [ ] L513: `<2> Fetch changes from `https://github.com/progit/progit2.git` and merge them into `master`.`
- [ ] L514: `<3> Push your `master` branch to `origin`.`
- [ ] L523: `$ git branch --set-upstream-to=progit/master master <3>`
- [ ] L529: `<2> Get a reference on progit's branches, in particular `master`.`
- [ ] L530: `<3> Set your `master` branch to fetch from the `progit` remote.`
- [ ] L537: `$ git checkout master <1>`
- [ ] L542: `<1> If you were on another branch, return to `master`.`
- [ ] L543: `<2> Fetch changes from `progit` and merge changes into `master`.`
- [ ] L544: `<3> Push your `master` branch to `origin`.`
- [ ] L547: `Git will happily do this work for you silently, but it won't warn you if you make a commit to `master`, pull from `progit`, then push to ...`
- [ ] L548: `So you'll have to take care never to commit directly to `master`, since that branch effectively belongs to the upstream repository.`

### `book/06-github/sections/3-maintaining.asc`

- [ ] L137: `10d539600d86723087810ec636870a504f4fee4d	refs/heads/master`
- [ ] L223: `Not only can you open Pull Requests that target the main or `master` branch, you can actually open a Pull Request targeting any branch in...`
- [ ] L358: `If you are using a branch other than "`master`" as your default branch that you want people to open Pull Requests on or see by default, y...`

### `book/07-git-tools/sections/advanced-merging.asc`

- [ ] L73: `Now we switch back to our `master` branch and add some documentation for the function.`
- [ ] L77: `$ git checkout master`
- [ ] L78: `Switched to branch 'master'`
- [ ] L95: `[master bec6336] Add comment documenting the function`
- [ ] L119: `## master`
- [ ] L125: `## master`
- [ ] L315: `* f1270f7 (HEAD, master) Update README`
- [ ] L325: `We now have three unique commits that live only on the `master` branch and three others that live on the `mundo` branch.`
- [ ] L535: `Let's say you started work on a topic branch, accidentally merged it into `master`, and now your commit history looks like this:`
- [ ] L554: `In this case, we want to move `master` to where it was before the merge commit (`C6`).`
- [ ] L571: `[master b1d8379] Revert "Merge branch 'topic'"`
- [ ] L584: `Git will get confused if you try to merge ``topic`` into ``master`` again:`
- [ ] L592: `There's nothing in `topic` that isn't already reachable from `master`.`
- [ ] L603: `[master 09f0126] Revert "Revert "Merge branch 'topic'""`
- [ ] L678: `For example, say you branched off a `release` branch and have done some work on it that you will want to merge back into your `master` br...`
- [ ] L679: `In the meantime some bugfix on `master` needs to be backported into your `release` branch.`
- [ ] L680: `You can merge the bugfix branch into the `release` branch and also `merge -s ours` the same branch into your `master` branch (even though...`

### `book/07-git-tools/sections/bundling.asc`

- [ ] L39: `$ git bundle create repo.bundle HEAD master`
- [ ] L47: `Now you have a file named `repo.bundle` that has all the data needed to re-create the repository's `master` branch.`
- [ ] L67: `If you don't include HEAD in the references, you have to also specify `-b master` or whatever branch is included because otherwise it won...`
- [ ] L87: `To get the three commits that we have in our `master` branch that weren't in the branch we originally cloned, we can use something like `...`
- [ ] L92: `$ git log --oneline master ^origin/master`
- [ ] L103: `$ git bundle create commits.bundle master ^9a466c5`
- [ ] L121: `71b84daaf49abed142a373b6e5c59a22dc6560dc refs/heads/master`
- [ ] L143: `71b84daaf49abed142a373b6e5c59a22dc6560dc refs/heads/master`
- [ ] L148: `Here we'll fetch the `master` branch of the bundle to a branch named `other-master` in our repository:`
- [ ] L152: `$ git fetch ../commits.bundle master:other-master`
- [ ] L154: `* [new branch]      master     -> other-master`
- [ ] L157: `Now we can see that we have the imported commits on the `other-master` branch as well as any commits we've done in the meantime in our ow...`
- [ ] L162: `* 8255d41 (HEAD, master) Third commit - first repo`
- [ ] L163: `\| * 71b84da (other-master) Last commit - second repo`

### `book/07-git-tools/sections/interactive-staging.asc`

- [ ] L154: `def log(treeish = 'master')`

### `book/07-git-tools/sections/replace.asc`

- [ ] L35: `Well, creating the historical history is easy, we can just put a branch in the history and then push that branch to the `master` branch o...`
- [ ] L41: `ef989d8 (HEAD, master) Fifth commit`
- [ ] L51: `Now we can push the new `history` branch to the `master` branch of our new repository:`
- [ ] L56: `$ git push project-history history:master`
- [ ] L64: `* [new branch]      history -> master`
- [ ] L74: `ef989d8 (HEAD, master) Fifth commit`
- [ ] L130: `$ git log --oneline master`
- [ ] L138: `* [new branch]      master     -> project-history/master`
- [ ] L141: `Now the collaborator would have their recent commits in the `master` branch and the historical commits in the `project-history/master` br...`
- [ ] L145: `$ git log --oneline master`
- [ ] L150: `$ git log --oneline project-history/master`
- [ ] L158: `So we want to replace the "fourth" commit in the `master` branch with the "fourth" commit in the `project-history/master` branch:`
- [ ] L165: `Now, if you look at the history of the `master` branch, it appears to look like this:`
- [ ] L169: `$ git log --oneline master`
- [ ] L203: `e146b5f14e79d4935160c0e83fb9ebe526b8da0d commit	refs/heads/master`
- [ ] L204: `c6e1e95051d41771a649f3145423f8809d1a74d4 commit	refs/remotes/history/master`
- [ ] L206: `e146b5f14e79d4935160c0e83fb9ebe526b8da0d commit	refs/remotes/origin/master`

### `book/07-git-tools/sections/rerere.asc`

- [ ] L63: `# On branch master`
- [ ] L142: `[master 68e16e5] Merge branch 'i18n'`
- [ ] L150: `Now, let's undo that merge and then rebase it on top of our `master` branch instead.`
- [ ] L167: `$ git rebase master`
- [ ] L255: `So, if you do a lot of re-merges, or want to keep a topic branch up to date with your `master` branch without a ton of merges, or you reb...`

### `book/07-git-tools/sections/reset.asc`

- [ ] L100: `Now we run `git init`, which will create a Git repository with a HEAD reference which points to the unborn `master` branch.`
- [ ] L112: `Then we run `git commit`, which takes the contents of the index and saves it as a permanent snapshot, creates a commit object which point...`
- [ ] L161: `This means if HEAD is set to the `master` branch (i.e. you're currently on the `master` branch), running `git reset 9e5e6a4` will start b...`
- [ ] L293: `For instance, say we have `master` and `develop` branches which point at different commits, and we're currently on `develop` (so HEAD poi...`
- [ ] L294: `If we run `git reset master`, `develop` itself will now point to the same commit that `master` does.`
- [ ] L295: `If we instead run `git checkout master`, `develop` does not move, HEAD itself does.`
- [ ] L296: `HEAD will now point to `master`.`

### `book/07-git-tools/sections/revision-selection.asc`

- [ ] L141: `For instance, to see where your `master` branch was yesterday, you can type:`
- [ ] L145: `$ git show master@{yesterday}`
- [ ] L148: `That would show you where the tip of your `master` branch was yesterday.`
- [ ] L155: `$ git log -g master`
- [ ] L157: `Reflog: master@{0} (Scott Chacon <schacon@gmail.com>)`
- [ ] L165: `Reflog: master@{1} (Scott Chacon <schacon@gmail.com>)`
- [ ] L248: `This syntax is useful only for merge commits, which have more than one parent -- the _first_ parent of a merge commit is from the branch ...`
- [ ] L313: `Say you want to see what is in your `experiment` branch that hasn't yet been merged into your `master` branch.`
- [ ] L314: `You can ask Git to show you a log of just those commits with `master..experiment` -- that means "`all commits reachable from `experiment`...`
- [ ] L319: `$ git log master..experiment`
- [ ] L324: `If, on the other hand, you want to see the opposite -- all commits in `master` that aren't in `experiment` -- you can reverse the branch ...`
- [ ] L325: ``experiment..master` shows you everything in `master` not reachable from `experiment`:`
- [ ] L329: `$ git log experiment..master`
- [ ] L339: `$ git log origin/master..HEAD`
- [ ] L342: `This command shows you any commits in your current branch that aren't in the `master` branch on your `origin` remote.`
- [ ] L343: `If you run a `git push` and your current branch is tracking `origin/master`, the commits listed by `git log origin/master..HEAD` are the ...`
- [ ] L345: `For example, you can get the same results as in the previous example by typing `git log origin/master..` -- Git substitutes `HEAD` if one...`
- [ ] L376: `If you want to see what is in `master` or `experiment` but not any common references, you can run:`
- [ ] L380: `$ git log master...experiment`
- [ ] L394: `$ git log --left-right master...experiment`

### `book/07-git-tools/sections/rewriting-history.asc`

- [ ] L356: `Ref 'refs/heads/master' was rewritten`
- [ ] L364: `It's generally a good idea to do this in a testing branch and then hard-reset your `master` branch after you've determined the outcome is...`
- [ ] L376: `Ref 'refs/heads/master' was rewritten`

### `book/07-git-tools/sections/signing.asc`

- [ ] L126: `[master 5c3386c] Signed commit`

### `book/07-git-tools/sections/stashing-cleaning.asc`

- [ ] L47: `"WIP on master: 049d078 Create index file"`
- [ ] L57: `# On branch master`
- [ ] L67: `stash@{0}: WIP on master: 049d078 Create index file`
- [ ] L68: `stash@{1}: WIP on master: c264051 Revert "Add file_size"`
- [ ] L69: `stash@{2}: WIP on master: 21d80a5 Add number to log`
- [ ] L80: `On branch master`
- [ ] L104: `On branch master`
- [ ] L123: `stash@{0}: WIP on master: 049d078 Create index file`
- [ ] L124: `stash@{1}: WIP on master: c264051 Revert "Add file_size"`
- [ ] L125: `stash@{2}: WIP on master: 21d80a5 Add number to log`
- [ ] L145: `Saved working directory and index state WIP on master: 1b65b17 added the index file`
- [ ] L165: `Saved working directory and index state WIP on master: 1b65b17 added the index file`
- [ ] L186: `+    def show(treeish = 'master')`
- [ ] L194: `Saved working directory and index state WIP on master: 1b65b17 added the index file`

### `book/07-git-tools/sections/submodules.asc`

- [ ] L47: `On branch master`
- [ ] L48: `Your branch is up-to-date with 'origin/master'.`
- [ ] L120: `[master fb9093c] Add DbConnector module`
- [ ] L133: `$ git push origin master`
- [ ] L227: `c3f01dc..d0354fc  master     -> origin/master`
- [ ] L228: `$ git merge origin/master`
- [ ] L261: `3f19983..d0354fc  master     -> origin/master`
- [ ] L291: `On branch master`
- [ ] L292: `Your branch is up-to-date with 'origin/master'.`
- [ ] L311: `On branch master`
- [ ] L312: `Your branch is up-to-date with 'origin/master'.`
- [ ] L388: `fb9093c..0a24cfc  master     -> origin/master`
- [ ] L399: `On branch master`
- [ ] L400: `Your branch is up-to-date with 'origin/master'.`
- [ ] L430: `On branch master`
- [ ] L431: `Your branch is up-to-date with 'origin/master'.`
- [ ] L463: `This means that there is no local working branch (like `master`, for example) tracking changes.`
- [ ] L632: `3d6d338..9a377d1  master -> master`
- [ ] L657: `9a377d1..eb974f8  master     -> origin/master`
- [ ] L717: `[master 9fd905e] merged our changes`
- [ ] L732: `[master 10d2c60] Merge Tom's Changes`
- [ ] L754: `$ git merge origin/master`
- [ ] L902: `$ git checkout master`
- [ ] L904: `Switched to branch 'master'`
- [ ] L905: `Your branch is up-to-date with 'origin/master'.`
- [ ] L908: `On branch master`
- [ ] L909: `Your branch is up-to-date with 'origin/master'.`
- [ ] L960: `$ git checkout --recurse-submodules master`
- [ ] L961: `Switched to branch 'master'`
- [ ] L962: `Your branch is up-to-date with 'origin/master'.`
- [ ] L965: `On branch master`
- [ ] L966: `Your branch is up-to-date with 'origin/master'.`
- [ ] L1015: `$ git checkout master`
- [ ] L1028: `$ git checkout -f master`
- [ ] L1030: `Switched to branch 'master'`

### `book/07-git-tools/sections/subtree-merges.asc`

- [ ] L24: `* [new branch]      master     -> rack_remote/master`
- [ ] L27: `$ git checkout -b rack_branch rack_remote/master`
- [ ] L28: `Branch rack_branch set up to track remote branch refs/remotes/rack_remote/master.`
- [ ] L32: `Now we have the root of the Rack project in our `rack_branch` branch and our own project in the `master` branch.`
- [ ] L40: `$ git checkout master`
- [ ] L41: `Switched to branch "master"`
- [ ] L50: `In this case, we want to pull the Rack project into our `master` project as a subdirectory.`
- [ ] L53: `We just switched back to your `master` branch, and we pull the `rack_branch` branch into the `rack` subdirectory of our `master` branch o...`
- [ ] L70: `Then, we can merge those changes back into our `master` branch.`
- [ ] L76: `$ git checkout master`
- [ ] L83: `You can also do the opposite -- make changes in the `rack` subdirectory of your `master` branch and then merge them into your `rack_branc...`
- [ ] L98: `Or, to compare what is in your `rack` subdirectory with what the `master` branch on the server was the last time you fetched, you can run:`
- [ ] L102: `$ git diff-tree -p rack_remote/master`

### `book/08-customizing-git/sections/config.asc`

- [ ] L108: `# On branch master`
- [ ] L180: `$ git chekcout master`
- [ ] L192: `$ git chekcout master`
- [ ] L210: `Git automatically colors most of its output, but there's a master switch if you don't like this behavior.`

### `book/08-customizing-git/sections/policy.asc`

- [ ] L223: `$ git push -f origin master`
- [ ] L230: `(refs/heads/master) (8338c5) (c5b616)`
- [ ] L233: `error: hook declined to update refs/heads/master`
- [ ] L235: `! [remote rejected] master -> master (hook declined)`
- [ ] L245: `(refs/heads/master) (fb8c72) (c56860)`
- [ ] L257: `error: hook declined to update refs/heads/master`
- [ ] L266: `! [remote rejected] master -> master (hook declined)`
- [ ] L324: `[master e05c914] Test [ref: 132]`

### `book/09-git-and-other-scms/sections/client-hg.asc`

- [ ] L60: `* ac7955c (HEAD, origin/master, origin/branches/default, origin/HEAD, refs/hg/origin/branches/default, refs/hg/origin/bookmarks/master, m...`
- [ ] L77: `│   └── master`
- [ ] L81: `│       │   └── master`
- [ ] L96: `For example, the `refs/hg/origin/branches/default` is a Git ref file that contains the SHA-1 starting with "`ac7955c`", which is the comm...`
- [ ] L112: `Notes for master`
- [ ] L124: `Once we dig down to one of the tree items, we find that inside it is a blob named "`ac9117f`" (the SHA-1 hash of the commit pointed to by...`
- [ ] L142: `Let's assume we've done some work and made some commits on the `master` branch, and you're ready to push it to the remote repository.`
- [ ] L148: `* ba04a2a (HEAD, master) Update makefile`
- [ ] L150: `* ac7955c (origin/master, origin/branches/default, origin/HEAD, refs/hg/origin/branches/default, refs/hg/origin/bookmarks/master) Create ...`
- [ ] L154: `Our `master` branch is two commits ahead of `origin/master`, but those two commits exist only on our local machine.`
- [ ] L161: `ac7955c..df85e87  master     -> origin/master`
- [ ] L165: `* d4c1038 Notes for master`
- [ ] L166: `* df85e87 (origin/master, origin/branches/default, origin/HEAD, refs/hg/origin/branches/default, refs/hg/origin/bookmarks/master) Add som...`
- [ ] L167: `\| * ba04a2a (HEAD, master) Update makefile`
- [ ] L175: `The rest is what we expected; `origin/master` has advanced by one commit, and our history has now diverged.`
- [ ] L180: `$ git merge origin/master`
- [ ] L186: `*   0c64627 (HEAD, master) Merge remote-tracking branch 'origin/master'`
- [ ] L188: `\| * df85e87 (origin/master, origin/branches/default, origin/HEAD, refs/hg/origin/branches/default, refs/hg/origin/bookmarks/master) Add s...`
- [ ] L203: `df85e87..0c64627  master -> master`
- [ ] L213: `\|\     Merge remote-tracking branch 'origin/master'`
- [ ] L280: `\|\     Merge remote-tracking branch 'origin/master'`
- [ ] L343: `\| \|  summary:     Merge remote-tracking branch 'origin/master'`
- [ ] L372: `\| \|\     Merge remote-tracking branch 'origin/master'`

### `book/09-git-and-other-scms/sections/client-p4.asc`

- [ ] L142: `[Talkhouse-master]`
- [ ] L143: `git-branch-name = master`
- [ ] L156: `git-branch-name = master`
- [ ] L204: `* master`
- [ ] L205: `remotes/origin/HEAD -> origin/master`
- [ ] L206: `remotes/origin/master`
- [ ] L210: `\| * d254865 (HEAD, origin/master, origin/HEAD, master) Upgrade to latest metrowerks on Beos -- the Intel one.`
- [ ] L223: `There are three branches, and Git has helpfully created a local `master` branch that tracks `origin/master`.`
- [ ] L230: `* cfd46ab (HEAD, master) Add documentation for new feature`
- [ ] L232: `* d254865 (origin/master, origin/HEAD) Upgrade to latest metrowerks on Beos -- the Intel one.`
- [ ] L248: `d254865..6afeb15  master     -> origin/master`
- [ ] L250: `* 6afeb15 (origin/master, origin/HEAD) Update copyright`
- [ ] L251: `\| * cfd46ab (HEAD, master) Add documentation for new feature`
- [ ] L266: `$ git merge origin/master`
- [ ] L285: `6afeb15..89cba2b  master -> master`
- [ ] L350: `Doing initial import of //depot/www/live/ from revision #head into refs/remotes/p4/master`
- [ ] L362: `* 70eaf78 (HEAD, p4/master, p4/HEAD, master) Initial import of //depot/www/live/ from the state at revision #head`
- [ ] L384: `* 018467c (HEAD, master) Change page title`
- [ ] L386: `* 70eaf78 (p4/master, p4/HEAD) Initial import of //depot/www/live/ from the state at revision #head`
- [ ] L396: `Performing incremental import into refs/remotes/p4/master git branch`
- [ ] L398: `Import destination: refs/remotes/p4/master`
- [ ] L401: `* 75cd059 (p4/master, p4/HEAD) Update copyright`
- [ ] L402: `\| * 018467c (HEAD, master) Change page title`
- [ ] L408: `Looks like they were, and `master` and `p4/master` have diverged.`
- [ ] L415: `Performing incremental import into refs/remotes/p4/master git branch`
- [ ] L418: `Rebasing the current branch onto remotes/p4/master`
- [ ] L426: `You can probably tell from the output, but `git p4 rebase` is a shortcut for `git p4 sync` followed by `git rebase p4/master`.`
- [ ] L430: `The `git p4 submit` command will try to create a new Perforce revision for every Git commit between `p4/master` and `master`.`
- [ ] L510: `Performing incremental import into refs/remotes/p4/master git branch`
- [ ] L512: `Import destination: refs/remotes/p4/master`
- [ ] L514: `Rebasing the current branch onto remotes/p4/master`
- [ ] L517: `* 775a46f (HEAD, p4/master, p4/HEAD, master) Change page title`
- [ ] L547: `* 3be6fd8 (HEAD, master) Correct email address`
- [ ] L548: `*   1dcbf21 Merge remote-tracking branch 'p4/master'`
- [ ] L550: `\| * c4689fc (p4/master, p4/HEAD) Grammar fix`
- [ ] L585: `* dadbd89 (HEAD, p4/master, p4/HEAD, master) Correct email address`
- [ ] L634: `* eae77ae (HEAD, p4/master, p4/HEAD, master) main`

### `book/09-git-and-other-scms/sections/client-svn.asc`

- [ ] L123: `* master`
- [ ] L139: `556a3e1e7ad1fde0a32823fc7e4d046bcfd86dae refs/heads/master`
- [ ] L153: `c3dcbe8488c6240392e8a5d7553bbffcb0f94ef0 refs/remotes/origin/master`
- [ ] L171: `[master 4af61fd] Adding git-svn instructions to the README`
- [ ] L226: `Current branch master is up to date.`
- [ ] L299: `Fast-forwarded master to refs/remotes/origin/trunk.`
- [ ] L312: `Suppose your history looks like the following: you created an `experiment` branch, did two commits, and then merged them back into `master`.`
- [ ] L372: `Now, if you want to merge your `opera` branch into `trunk` (your `master` branch), you can do so with a normal `git merge`.`

### `book/09-git-and-other-scms/sections/import-custom.asc`

- [ ] L119: `puts 'commit refs/heads/master'`
- [ ] L246: `puts 'commit refs/heads/master'`
- [ ] L279: `commit refs/heads/master`
- [ ] L289: `commit refs/heads/master`
- [ ] L359: `To get them, you must reset your branch to where `master` is now:`
- [ ] L364: `$ git reset --hard master`

### `book/09-git-and-other-scms/sections/import-hg.asc`

- [ ] L74: `master: Exporting full revision 1/22208 with 13/0/0 added/changed/removed files`
- [ ] L75: `master: Exporting simple delta revision 2/22208 with 1/1/0 added/changed/removed files`
- [ ] L76: `master: Exporting simple delta revision 3/22208 with 0/1/0 added/changed/removed files`
- [ ] L78: `master: Exporting simple delta revision 22206/22208 with 0/4/0 added/changed/removed files`
- [ ] L79: `master: Exporting simple delta revision 22207/22208 with 0/2/0 added/changed/removed files`
- [ ] L80: `master: Exporting thorough delta revision 22208/22208 with 3/213/0 added/changed/removed files`

### `book/09-git-and-other-scms/sections/import-p4.asc`

- [ ] L41: `Import destination: refs/remotes/p4/master`
- [ ] L81: `Ref 'refs/heads/master' was rewritten`

### `book/09-git-and-other-scms/sections/import-svn.asc`

- [ ] L120: `Unfortunately, `git svn` creates an extra branch named `trunk`, which maps to Subversion's default branch, but the `trunk` ref points to ...`
- [ ] L121: `Since `master` is more idiomatically Git, here's how to remove the extra branch:`

### `book/10-git-internals/sections/environment.asc`

- [ ] L153: `On branch master`
- [ ] L154: `Your branch is up-to-date with 'origin/master'.`
- [ ] L165: `20:15:14.867079 pkt-line.c:46           packet:          git< 97b8860c071898d9e162678ea1035a8ced2f8b1f HEAD\0multi_ack thin-pack side-ban...`
- [ ] L203: `On branch master`
- [ ] L204: `Your branch is up-to-date with 'origin/master'.`
- [ ] L234: `[master 9e3d55a] My message`

### `book/10-git-internals/sections/maintenance.asc`

- [ ] L32: `.git/refs/heads/master`
- [ ] L45: `ab1afef80fac8e34258ff41fc1b867c702daa24b refs/heads/master`
- [ ] L65: `Here's an example that hard-resets the `master` branch in your test repository to an older commit and then recovers the lost commits.`
- [ ] L78: `Now, move the `master` branch back to the middle commit:`
- [ ] L145: `Cool – now you have a branch named `recover-branch` that is where your `master` branch used to be, making the first two commits reachable...`
- [ ] L198: `[master 7b30847] Add git tarball`
- [ ] L211: `[master dadf725] Oops - remove large tarball`
- [ ] L294: `Ref 'refs/heads/master' was rewritten`

### `book/10-git-internals/sections/objects.asc`

- [ ] L133: `$ git cat-file -p master^{tree}`
- [ ] L139: `The `master^{tree}` syntax specifies the tree object that is pointed to by the last commit on your `master` branch.`
- [ ] L150: `Depending on what shell you use, you may encounter errors when using the `master^{tree}` syntax.`
- [ ] L152: `In CMD on Windows, the `^` character is used for escaping, so you have to double it to avoid this: `git cat-file -p master^^{tree}`.`
- [ ] L153: `When using PowerShell, parameters using `{}` characters have to be quoted to avoid the parameter being parsed incorrectly: `git cat-file ...`
- [ ] L155: `If you're using ZSH, the `^` character is used for globbing, so you have to enclose the whole expression in quotes: `git cat-file -p "mas...`

### `book/10-git-internals/sections/packfiles.asc`

- [ ] L28: `$ git checkout master`
- [ ] L31: `[master 484a592] Create repo.rb`
- [ ] L42: `$ git cat-file -p master^{tree}`
- [ ] L62: `[master 2431da6] Modify repo.rb a bit`
- [ ] L70: `$ git cat-file -p master^{tree}`

### `book/10-git-internals/sections/refs.asc`

- [ ] L23: `$ echo 1a410efbd13591db07496601ebc7a059dd55cfe9 > .git/refs/heads/master`
- [ ] L30: `$ git log --pretty=oneline master`
- [ ] L40: `$ git update-ref refs/heads/master 1a410efbd13591db07496601ebc7a059dd55cfe9`
- [ ] L84: `ref: refs/heads/master`
- [ ] L103: `refs/heads/master`
- [ ] L185: `For instance, you can add a remote called `origin` and push your `master` branch to it:`
- [ ] L190: `$ git push origin master`
- [ ] L196: `a11bef0..ca82a6d  master -> master`
- [ ] L199: `Then, you can see what the `master` branch on the `origin` remote was the last time you communicated with the server, by checking the `re...`
- [ ] L203: `$ cat .git/refs/remotes/origin/master`

### `book/10-git-internals/sections/refspec.asc`

- [ ] L25: `So, if there is a `master` branch on the server, you can access the log of that branch locally via any of the following:`
- [ ] L29: `$ git log origin/master`
- [ ] L30: `$ git log remotes/origin/master`
- [ ] L31: `$ git log refs/remotes/origin/master`
- [ ] L34: `They're all equivalent, because Git expands each of them to `refs/remotes/origin/master`.`
- [ ] L36: `If you want Git instead to pull down only the `master` branch each time, and not every other branch on the remote server, you can change ...`
- [ ] L40: `fetch = +refs/heads/master:refs/remotes/origin/master`
- [ ] L45: `To pull the `master` branch on the remote down to `origin/mymaster` locally, you can run:`
- [ ] L49: `$ git fetch origin master:refs/remotes/origin/mymaster`
- [ ] L57: `$ git fetch origin master:refs/remotes/origin/mymaster \`
- [ ] L60: `! [rejected]        master     -> origin/mymaster  (non fast forward)`
- [ ] L64: `In this case, the `master` branch pull was rejected because it wasn't listed as a fast-forward reference.`
- [ ] L68: `If you want to always fetch the `master` and `experiment` branches from the `origin` remote, add two lines:`
- [ ] L74: `fetch = +refs/heads/master:refs/remotes/origin/master`
- [ ] L86: `If you have a QA team that pushes a series of branches, and you want to get the `master` branch and any of the QA team's branches but not...`
- [ ] L92: `fetch = +refs/heads/master:refs/remotes/origin/master`
- [ ] L104: `If the QA team wants to push their `master` branch to `qa/master` on the remote server, they can run:`
- [ ] L108: `$ git push origin master:refs/heads/qa/master`
- [ ] L118: `push = refs/heads/master:refs/heads/qa/master`
- [ ] L121: `Again, this will cause a `git push origin` to push the local `master` branch to the remote `qa/master` branch by default.`

### `book/10-git-internals/sections/transfer-protocols.asc`

- [ ] L31: `ca82a6dff817ec66f44342007202690a93763949     refs/heads/master`
- [ ] L40: `ref: refs/heads/master`
- [ ] L43: `You need to check out the `master` branch when you've completed the process.`
- [ ] L124: `Git checks out a working copy of the `master` branch that was pointed to by the HEAD reference you downloaded at the beginning.`
- [ ] L140: `For example, say you run `git push origin master` in your project, and `origin` is defined as a URL that uses the SSH protocol.`
- [ ] L147: `00a5ca82a6dff817ec66f4437202690a93763949 refs/heads/master□report-status \`
- [ ] L153: `The `git-receive-pack` command immediately responds with one line for each reference it currently has – in this case, just the `master` b...`
- [ ] L164: `For instance, if you're updating the `master` branch and adding an `experiment` branch, the `send-pack` response may look something like ...`
- [ ] L169: `refs/heads/master report-status`
- [ ] L197: `00ab6c5f0e45abd7832bf23074a333f739977c9e8188 refs/heads/master□report-status \`
- [ ] L237: `multi_ack_detailed symref=HEAD:refs/heads/master \`
- [ ] L239: `003fe2409a098dc3e53539a9028a94b6224db9d6a6b6 refs/heads/master`
- [ ] L244: `In addition, it sends back what HEAD points to (`symref=HEAD:refs/heads/master`) so the client knows what to check out if this is a clone.`
- [ ] L269: `multi_ack_detailed no-done symref=HEAD:refs/heads/master \`
- [ ] L271: `003fca82a6dff817ec66f44342007202690a93763949 refs/heads/master`

### `book/B-embedding-git/sections/jgit.asc`

- [ ] L64: `Ref master = repo.getRef("master");`
- [ ] L67: `ObjectId masterTip = master.getObjectId();`
- [ ] L93: `The first line gets a pointer to the `master` reference.`
- [ ] L94: `JGit automatically grabs the _actual_ `master` ref, which lives at `refs/heads/master`, and returns an object that lets you fetch informa...`
- [ ] L98: `The second line gets the target of the `master` reference, which is returned as an ObjectId instance.`

### `book/introduction.asc`

- [ ] L31: `This chapter will round out your knowledge of Git so that you are truly a master.`
