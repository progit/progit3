#!/usr/bin/env node
// Render the Pro Git AsciiDoc sources into per-section pages for the Astro site.
//
// This is the same strategy as builder/src/site.rs: convert progit.asc to a
// single HTML document with Asciidoctor, then split it into one page per
// section by walking the highly regular HTML5 output (`<div class="sect1">`
// chapters containing `<div class="sect2">` sections). Literal `<div` only
// ever appears as a real tag — inside code listings it is escaped to `&lt;div`
// — so counting div nesting is reliable without a full HTML parser.
//
// Output:
//   src/generated/book.json   — chapters, pages, and metadata
//   public/images/            — copy of the repo's images/ directory
//   public/book-cover.png     — copy of book/cover.png
//   public/favicon.ico        — copy of Pro.ico

import { convert } from 'asciidoctor';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const repoRoot = path.resolve(siteRoot, '..');
const base = (process.env.BASE_PATH ?? '/').replace(/\/+$/, '');

// ---------------------------------------------------------------------------
// 1. Make sure book/contributors.txt exists (mirrors the Rakefile task).
// ---------------------------------------------------------------------------

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function ensureContributors() {
  const file = path.join(repoRoot, 'book', 'contributors.txt');
  if (fs.existsSync(file)) return;
  let body = 'Contributors list unavailable in this build.\n';
  try {
    const head = git(['rev-parse', '--short', 'HEAD']);
    const shortlog = git(['shortlog', '-s', 'HEAD']);
    const names = shortlog
      .split('\n')
      .map((line) => line.replace(/^\s*\d+\t/, '').trim())
      .filter((name) => name && !/(Straub|Chacon|dependabot)/.test(name))
      .sort((a, b) => a.localeCompare(b));
    body = `Contributors as of ${head}:\n\n${names.join('\n')}\n`;
  } catch {
    // Shallow clone or no git: keep the placeholder.
  }
  fs.writeFileSync(file, body);
  console.log(`generated ${path.relative(repoRoot, file)}`);
}

// ---------------------------------------------------------------------------
// 2. Convert the whole book to embedded HTML with Asciidoctor.js.
// ---------------------------------------------------------------------------

async function convertBook() {
  let revnumber = '3.0-dev';
  try {
    const tag = git(['describe', '--tags', '--abbrev=0']);
    const [major, minor, patch] = tag.split('.');
    if (patch !== undefined) revnumber = `${major}.${minor}.${Number(patch) + 1}`;
  } catch {
    // No tags available: keep the default.
  }
  const revdate = new Date().toISOString().slice(0, 10);

  const source = fs.readFileSync(path.join(repoRoot, 'progit.asc'), 'utf8');
  const html = await convert(source, {
    safe: 'unsafe',
    base_dir: repoRoot,
    standalone: false,
    doctype: 'book',
    attributes: {
      revnumber,
      revdate,
      // The document header sets `icons: font` (Font Awesome); render plain
      // text admonition labels instead so no icon font is needed.
      'icons!': '',
      // The site builds its own table of contents from the parsed chapters.
      'toc!': '',
    },
  });
  return { html: String(html), revnumber, revdate };
}

// ---------------------------------------------------------------------------
// 3. HTML block helpers (ports of the builder/src/site.rs helpers).
// ---------------------------------------------------------------------------

/** Given `open` pointing at a `<div`, return the index just past its matching `</div>`. */
function divEnd(s, open) {
  let i = open;
  let depth = 0;
  for (;;) {
    const nextOpen = s.indexOf('<div', i);
    const nextClose = s.indexOf('</div>', i);
    if (nextOpen !== -1 && (nextClose === -1 || nextOpen < nextClose)) {
      depth += 1;
      i = nextOpen + 4;
    } else if (nextClose !== -1) {
      depth -= 1;
      i = nextClose + 6;
      if (depth <= 0) return i;
    } else {
      return s.length;
    }
  }
}

/** Given `open` at a `<div`, return [innerHtml, indexPastClose]. */
function divInner(s, open) {
  const end = divEnd(s, open);
  const gt = s.indexOf('>', open);
  const contentStart = gt === -1 ? open : gt + 1;
  const innerEnd = Math.max(end - 6, contentStart);
  return [s.slice(contentStart, innerEnd), end];
}

/** Inner HTML of the first `<div class="sectionbody">` within `s`. */
function bodyOf(s) {
  const open = s.indexOf('<div class="sectionbody">');
  if (open === -1) return null;
  return divInner(s, open)[0];
}

/** Parse an `<hN id="…">Title</hN>` heading, returning { id, title } or null. */
function heading(s, level) {
  const open = `<h${level} id="`;
  let i = s.indexOf(open);
  if (i === -1) return null;
  i += open.length;
  const idEnd = s.indexOf('"', i);
  if (idEnd === -1) return null;
  const id = s.slice(i, idEnd);
  const gt = s.indexOf('>', idEnd);
  if (gt === -1) return null;
  const close = `</h${level}>`;
  const tEnd = s.indexOf(close, gt);
  if (tEnd === -1) return null;
  return { id, title: stripTags(s.slice(gt + 1, tEnd)) };
}

function stripTags(s) {
  return s.replace(/<[^>]*>/g, '').trim();
}

function esc(s) {
  return s.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
}

function slugOf(id) {
  return id.replace(/^_+/, '');
}

// ---------------------------------------------------------------------------
// 4. Split into chapters/sections and build the ordered page list.
// ---------------------------------------------------------------------------

function parseChapters(content) {
  const chapters = [];
  let pos = 0;
  for (;;) {
    const open = content.indexOf('<div class="sect1">', pos);
    if (open === -1) break;
    const [inner, after] = divInner(content, open);
    pos = after;

    const h = heading(inner, 2);
    if (!h) continue;

    const body = bodyOf(inner) ?? inner;

    const sections = [];
    const firstSect2 = body.indexOf('<div class="sect2">');
    let bpos = 0;
    for (;;) {
      const sOpen = body.indexOf('<div class="sect2">', bpos);
      if (sOpen === -1) break;
      const sEnd = divEnd(body, sOpen);
      const block = body.slice(sOpen, sEnd);
      bpos = sEnd;
      const sh = heading(block, 3);
      if (sh) {
        sections.push({ id: sh.id, slug: slugOf(sh.id), title: sh.title, html: block });
      }
    }

    const introHtml = firstSect2 === -1 ? body : body.slice(0, firstSect2);
    const landing = sections.length > 0 ? sections[0].slug : slugOf(h.id);

    chapters.push({ id: h.id, title: h.title, introHtml, sections, landing });
  }
  return chapters;
}

function buildPages(chapters) {
  const pages = [];
  for (const chapter of chapters) {
    if (chapter.sections.length === 0) {
      // Front matter (License, Introduction, …): a single page.
      pages.push({
        slug: slugOf(chapter.id),
        title: chapter.title,
        chapterTitle: chapter.title,
        chapterLanding: chapter.landing,
        body:
          `<div class="sect1">\n<h2 id="${esc(chapter.id)}">${esc(chapter.title)}</h2>\n` +
          `<div class="sectionbody">${chapter.introHtml}</div>\n</div>`,
      });
      continue;
    }

    chapter.sections.forEach((section, idx) => {
      let body = '<div class="sect1">\n';
      if (idx === 0) {
        body += `<h2 id="${esc(chapter.id)}">${esc(chapter.title)}</h2>\n`;
      }
      body += '<div class="sectionbody">';
      if (idx === 0 && chapter.introHtml.trim() !== '') {
        body += chapter.introHtml;
      }
      body += section.html;
      body += '</div>\n</div>';

      pages.push({
        slug: section.slug,
        title: section.title,
        chapterTitle: chapter.title,
        chapterLanding: chapter.landing,
        body,
      });
    });
  }
  return pages;
}

// ---------------------------------------------------------------------------
// 5. Rewrite in-document links and image paths.
// ---------------------------------------------------------------------------

function indexIds(pages) {
  const map = new Map();
  for (const page of pages) {
    for (const match of page.body.matchAll(/id="([^"]+)"/g)) {
      if (!map.has(match[1])) map.set(match[1], page.slug);
    }
  }
  return map;
}

function rewriteLinks(html, idToSlug) {
  const pageHref = (id, whole) => {
    const slug = idToSlug.get(id);
    return slug ? `href="${base}/book/${slug}/#${id}"` : whole;
  };
  return (
    html
      .replaceAll('src="images/', `src="${base}/images/`)
      .replaceAll('href="images/', `href="${base}/images/`)
      // In-document cross-references.
      .replace(/href="#([^"]+)"/g, (whole, id) => pageHref(id, whole))
      // Inter-file cross-references (`<<chNN-…#id>>`) render as `file.html#id`
      // in the single-document conversion; resolve them by anchor id. A bare
      // `file.html` link targets the chapter whose anchor id equals the
      // file's base name.
      .replace(/href="([\w-]+)\.html(?:#([^"]+))?"/g, (whole, file, id) =>
        pageHref(id ?? file, whole),
      )
  );
}

// ---------------------------------------------------------------------------
// 6. Static assets.
// ---------------------------------------------------------------------------

function copyAssets() {
  const publicDir = path.join(siteRoot, 'public');
  fs.mkdirSync(publicDir, { recursive: true });
  fs.cpSync(path.join(repoRoot, 'images'), path.join(publicDir, 'images'), {
    recursive: true,
  });
  fs.copyFileSync(path.join(repoRoot, 'book', 'cover.png'), path.join(publicDir, 'book-cover.png'));
  fs.copyFileSync(path.join(repoRoot, 'Pro.ico'), path.join(publicDir, 'favicon.ico'));
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

ensureContributors();

console.log('converting progit.asc with Asciidoctor.js …');
const { html, revnumber, revdate } = await convertBook();

const chapters = parseChapters(html);
if (chapters.length === 0) {
  console.error('no chapters found in the Asciidoctor output');
  process.exit(1);
}

const pages = buildPages(chapters);
const idToSlug = indexIds(pages);
for (const page of pages) {
  page.body = rewriteLinks(page.body, idToSlug);
}

const book = {
  title: 'Pro Git',
  revnumber,
  revdate,
  chapters: chapters.map((c) => ({
    slug: slugOf(c.id),
    title: c.title,
    landing: c.landing,
    sections: c.sections.map((s) => ({ slug: s.slug, title: s.title })),
  })),
  pages,
};

const outDir = path.join(siteRoot, 'src', 'generated');
fs.mkdirSync(outDir, { recursive: true });
fs.writeFileSync(path.join(outDir, 'book.json'), JSON.stringify(book));
console.log(`wrote src/generated/book.json (${chapters.length} chapters, ${pages.length} pages)`);

copyAssets();
console.log('copied images/, book cover, and favicon into public/');
