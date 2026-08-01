//! Split the single-file Asciidoctor HTML into a multi-page reader — one page
//! per section, with a sidebar table of contents and prev/next navigation, in
//! the spirit of the online book at <https://git-scm.com/book>.
//!
//! Asciidoctor's HTML5 output is highly regular, which makes a reliable split
//! possible without a full HTML parser: `<div id="content">` holds sibling
//! `<div class="sect1">` chapters, each with an `<h2 id>` heading and
//! `<div class="sect2">` sections. We walk those blocks by counting `<div>`
//! nesting (literal `<div` only ever appears as a real tag — inside code
//! listings it is escaped to `&lt;div`).

use std::collections::HashMap;

use crate::error::{BuildError, Result};

/// A fully rendered, ready-to-serve site keyed by request path.
pub struct Site {
    routes: HashMap<String, Resource>,
    /// Number of reader pages (for logging).
    pub page_count: usize,
}

struct Resource {
    ctype: &'static str,
    body: Vec<u8>,
}

impl Site {
    /// Serve a route, if this path is one the site owns.
    pub fn get(&self, path: &str) -> Option<(&'static str, &[u8])> {
        self.routes.get(path).map(|r| (r.ctype, r.body.as_slice()))
    }

    /// Build the whole site from the single-file HTML document.
    pub fn build(full_html: &str) -> Result<Site> {
        let book_title = between(full_html, "<h1>", "</h1>")
            .map(strip_tags)
            .unwrap_or_else(|| "Pro Git".to_string());
        let default_css = between(full_html, "<style>", "</style>")
            .ok_or_else(|| BuildError::new("could not find the stylesheet in the HTML output"))?;

        let content = content_inner(full_html)
            .ok_or_else(|| BuildError::new("could not find <div id=\"content\"> in the HTML"))?;

        let chapters = parse_chapters(content);
        if chapters.is_empty() {
            return Err(BuildError::new("no chapters found in the HTML output"));
        }

        // Flatten into reader pages in document order.
        let mut pages = build_pages(&chapters);

        // Map every anchor id to the page that now contains it, then rewrite
        // in-document cross-references and image paths on every page.
        let id_to_path = index_ids(&pages);
        for page in &mut pages {
            page.body = rewrite_links(&page.body, &id_to_path);
        }

        // Assemble routes.
        let mut routes = HashMap::new();
        let css = format!("{default_css}\n\n{}", LAYOUT_CSS);
        routes.insert(
            "/_assets/style.css".to_string(),
            Resource {
                ctype: "text/css; charset=utf-8",
                body: css.into_bytes(),
            },
        );

        for (i, page) in pages.iter().enumerate() {
            let prev = i.checked_sub(1).map(|j| &pages[j]);
            let next = pages.get(i + 1);
            let sidebar = render_sidebar(&chapters, &page.path);
            let html = render_page(&book_title, page, prev, next, &sidebar);
            routes.insert(
                page.path.clone(),
                Resource {
                    ctype: "text/html; charset=utf-8",
                    body: html.into_bytes(),
                },
            );
        }

        let index = render_index(&book_title, &chapters);
        routes.insert(
            "/".to_string(),
            Resource {
                ctype: "text/html; charset=utf-8",
                body: index.into_bytes(),
            },
        );

        Ok(Site {
            routes,
            page_count: pages.len(),
        })
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

struct Chapter {
    id: String,
    title: String,
    intro_html: String,
    sections: Vec<Section>,
    /// The chapter's own page path (its first section, or itself when it has none).
    landing: String,
}

struct Section {
    title: String,
    html: String,
    path: String,
}

struct Page {
    path: String,
    title: String,
    body: String,
}

/// Extract the inner HTML of `<div id="content"> … </div>`.
fn content_inner(html: &str) -> Option<&str> {
    let open = html.find("<div id=\"content\">")?;
    let (inner, _) = div_inner(html, open);
    Some(inner)
}

fn parse_chapters(content: &str) -> Vec<Chapter> {
    let mut chapters = Vec::new();
    let mut pos = 0;
    while let Some(rel) = content[pos..].find("<div class=\"sect1\">") {
        let open = pos + rel;
        let (inner, after) = div_inner(content, open);
        pos = after;

        let (id, title) = match heading(inner, 2) {
            Some(v) => v,
            None => continue,
        };

        // The chapter body lives in <div class="sectionbody"> …
        let body = body_of(inner).unwrap_or(inner);

        // Sections are top-level <div class="sect2"> blocks; anything before the
        // first one is the chapter's introduction.
        let mut sections = Vec::new();
        let mut bpos = 0;
        let first_sect2 = body.find("<div class=\"sect2\">");
        while let Some(rel) = body[bpos..].find("<div class=\"sect2\">") {
            let s_open = bpos + rel;
            let s_end = div_end(body, s_open);
            let block = &body[s_open..s_end];
            bpos = s_end;
            if let Some((sid, stitle)) = heading(block, 3) {
                sections.push(Section {
                    title: stitle,
                    html: block.to_string(),
                    path: page_path(&sid),
                });
            }
        }

        let intro_html = match first_sect2 {
            Some(idx) => body[..idx].to_string(),
            None => body.to_string(),
        };

        let landing = sections
            .first()
            .map(|s| s.path.clone())
            .unwrap_or_else(|| page_path(&id));

        chapters.push(Chapter {
            id,
            title,
            intro_html,
            sections,
            landing,
        });
    }
    chapters
}

/// Turn the parsed chapters into the ordered list of reader pages.
fn build_pages(chapters: &[Chapter]) -> Vec<Page> {
    let mut pages = Vec::new();
    for chapter in chapters {
        if chapter.sections.is_empty() {
            // A front-matter section (License, Introduction, …): one page.
            let body = format!(
                "<div class=\"sect1\">\n<h2 id=\"{id}\">{title}</h2>\n\
                 <div class=\"sectionbody\">{intro}</div>\n</div>",
                id = esc(&chapter.id),
                title = esc(&chapter.title),
                intro = chapter.intro_html,
            );
            pages.push(Page {
                path: page_path(&chapter.id),
                title: chapter.title.clone(),
                body,
            });
            continue;
        }

        for (idx, section) in chapter.sections.iter().enumerate() {
            let mut body = String::new();
            body.push_str(&breadcrumb(chapter));
            body.push_str("<div class=\"sect1\">\n");
            if idx == 0 {
                body.push_str(&format!(
                    "<h2 id=\"{}\">{}</h2>\n",
                    esc(&chapter.id),
                    esc(&chapter.title)
                ));
            }
            body.push_str("<div class=\"sectionbody\">");
            if idx == 0 && !chapter.intro_html.trim().is_empty() {
                body.push_str(&chapter.intro_html);
            }
            body.push_str(&section.html);
            body.push_str("</div>\n</div>");

            pages.push(Page {
                path: section.path.clone(),
                title: section.title.clone(),
                body,
            });
        }
    }
    pages
}

// ---------------------------------------------------------------------------
// Link handling
// ---------------------------------------------------------------------------

/// Map every `id="…"` occurrence to the path of the page it now lives on.
fn index_ids(pages: &[Page]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for page in pages {
        for id in find_ids(&page.body) {
            map.entry(id).or_insert_with(|| page.path.clone());
        }
    }
    map
}

fn find_ids(html: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut pos = 0;
    while let Some(rel) = html[pos..].find("id=\"") {
        let start = pos + rel + 4;
        if let Some(end_rel) = html[start..].find('"') {
            ids.push(html[start..start + end_rel].to_string());
            pos = start + end_rel + 1;
        } else {
            break;
        }
    }
    ids
}

/// Rewrite in-document links (`href="#id"`) to point at the owning page, and
/// make image sources absolute so they resolve from any page URL.
fn rewrite_links(html: &str, id_to_path: &HashMap<String, String>) -> String {
    let with_images = html
        .replace("src=\"images/", "src=\"/images/")
        .replace("href=\"images/", "href=\"/images/");

    let mut out = String::with_capacity(with_images.len());
    let mut pos = 0;
    let needle = "href=\"#";
    while let Some(rel) = with_images[pos..].find(needle) {
        let start = pos + rel;
        out.push_str(&with_images[pos..start]);
        let id_start = start + needle.len();
        let id_end = with_images[id_start..]
            .find('"')
            .map(|p| id_start + p)
            .unwrap_or(with_images.len());
        let id = &with_images[id_start..id_end];
        match id_to_path.get(id) {
            Some(path) => out.push_str(&format!("href=\"{path}#{id}\"")),
            None => out.push_str(&format!("href=\"#{id}\"")),
        }
        pos = id_end + 1;
    }
    out.push_str(&with_images[pos..]);
    out
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn breadcrumb(chapter: &Chapter) -> String {
    format!(
        "<div class=\"breadcrumb\"><a href=\"{path}\">{title}</a></div>\n",
        path = chapter.landing,
        title = esc(&chapter.title),
    )
}

fn render_sidebar(chapters: &[Chapter], current: &str) -> String {
    let mut s = String::from("<nav class=\"book-toc\" aria-label=\"Table of contents\">\n");
    s.push_str("<a class=\"book-toc-title\" href=\"/\">Pro Git</a>\n<ul>\n");
    for chapter in chapters {
        let active = if chapter.landing == current || chapter.sections.iter().any(|x| x.path == current) {
            " class=\"open\""
        } else {
            ""
        };
        s.push_str(&format!(
            "<li{active}><a class=\"chap{cur}\" href=\"{path}\">{title}</a>",
            active = active,
            cur = current_class(&chapter.landing, current, chapter.sections.is_empty()),
            path = chapter.landing,
            title = esc(&chapter.title),
        ));
        if !chapter.sections.is_empty() {
            s.push_str("\n<ul>\n");
            for section in &chapter.sections {
                s.push_str(&format!(
                    "<li><a class=\"sec{cur}\" href=\"{path}\">{title}</a></li>\n",
                    cur = current_class(&section.path, current, true),
                    path = section.path,
                    title = esc(&section.title),
                ));
            }
            s.push_str("</ul>");
        }
        s.push_str("</li>\n");
    }
    s.push_str("</ul>\n</nav>\n");
    s
}

fn current_class(path: &str, current: &str, exact: bool) -> &'static str {
    if exact && path == current {
        " current"
    } else {
        ""
    }
}

fn render_page(
    book_title: &str,
    page: &Page,
    prev: Option<&Page>,
    next: Option<&Page>,
    sidebar: &str,
) -> String {
    let nav = render_pager(prev, next);
    format!(
        "{head}\
<div class=\"layout\">\n{sidebar}\
<main class=\"main\">\n\
<div class=\"pager top\">{nav}</div>\n\
<article class=\"content\">\n{body}\n</article>\n\
<div class=\"pager bottom\">{nav}</div>\n\
</main>\n</div>\n{foot}",
        head = html_head(&format!("{} · {book_title}", page.title)),
        sidebar = sidebar,
        nav = nav,
        body = page.body,
        foot = HTML_FOOT,
    )
}

fn render_pager(prev: Option<&Page>, next: Option<&Page>) -> String {
    let prev_html = match prev {
        Some(p) => format!(
            "<a class=\"prev\" href=\"{}\">← {}</a>",
            p.path,
            esc(&p.title)
        ),
        None => "<span class=\"prev disabled\"></span>".to_string(),
    };
    let home = "<a class=\"home\" href=\"/\">Contents</a>";
    let next_html = match next {
        Some(n) => format!(
            "<a class=\"next\" href=\"{}\">{} →</a>",
            n.path,
            esc(&n.title)
        ),
        None => "<span class=\"next disabled\"></span>".to_string(),
    };
    format!("{prev_html}{home}{next_html}")
}

fn render_index(book_title: &str, chapters: &[Chapter]) -> String {
    let mut list = String::from("<ul class=\"toc-index\">\n");
    for chapter in chapters {
        list.push_str(&format!(
            "<li><a class=\"chap\" href=\"{path}\">{title}</a>",
            path = chapter.landing,
            title = esc(&chapter.title),
        ));
        if !chapter.sections.is_empty() {
            list.push_str("\n<ul>\n");
            for section in &chapter.sections {
                list.push_str(&format!(
                    "<li><a href=\"{path}\">{title}</a></li>\n",
                    path = section.path,
                    title = esc(&section.title),
                ));
            }
            list.push_str("</ul>");
        }
        list.push_str("</li>\n");
    }
    list.push_str("</ul>\n");

    format!(
        "{head}\
<div class=\"layout index-layout\">\n\
<main class=\"main index\">\n\
<div class=\"cover\"><img src=\"/book/cover.png\" alt=\"Pro Git cover\"></div>\n\
<h1>{title}</h1>\n\
<p class=\"lede\">Read the book one section at a time. Pick a chapter to begin.</p>\n\
{list}\
</main>\n</div>\n{foot}",
        head = html_head(book_title),
        title = esc(book_title),
        list = list,
        foot = HTML_FOOT,
    )
}

fn html_head(title: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\
<meta charset=\"utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
<title>{title}</title>\n\
<link rel=\"stylesheet\" href=\"/_assets/style.css\">\n\
</head>\n<body class=\"book reader\">\n",
        title = esc_no_amp(title),
    )
}

const HTML_FOOT: &str = "</body>\n</html>\n";

// ---------------------------------------------------------------------------
// Low-level HTML block helpers
// ---------------------------------------------------------------------------

/// Given `open` pointing at a `<div`, return the index just past its matching
/// `</div>`, tracking nested divs.
fn div_end(s: &str, open: usize) -> usize {
    let mut i = open;
    let mut depth = 0usize;
    loop {
        let next_open = s[i..].find("<div");
        let next_close = s[i..].find("</div>");
        match (next_open, next_close) {
            (Some(o), Some(c)) if o < c => {
                depth += 1;
                i += o + 4;
            }
            (_, Some(c)) => {
                depth = depth.saturating_sub(1);
                i += c + 6;
                if depth == 0 {
                    return i;
                }
            }
            _ => return s.len(),
        }
    }
}

/// Given `open` at a `<div`, return (inner_html, index_past_close).
fn div_inner(s: &str, open: usize) -> (&str, usize) {
    let end = div_end(s, open);
    let content_start = s[open..end]
        .find('>')
        .map(|p| open + p + 1)
        .unwrap_or(open);
    let inner_end = end.saturating_sub(6); // strip trailing "</div>"
    (&s[content_start..inner_end.max(content_start)], end)
}

/// Inner HTML of the first `<div class="sectionbody">` within `s`.
fn body_of(s: &str) -> Option<&str> {
    let open = s.find("<div class=\"sectionbody\">")?;
    let (inner, _) = div_inner(s, open);
    Some(inner)
}

/// Parse an `<hN id="…">Title</hN>` heading, returning (id, plain title).
fn heading(s: &str, level: u8) -> Option<(String, String)> {
    let open = format!("<h{level} id=\"");
    let i = s.find(&open)? + open.len();
    let id_end = s[i..].find('"')? + i;
    let id = s[i..id_end].to_string();
    let gt = s[id_end..].find('>')? + id_end + 1;
    let close = format!("</h{level}>");
    let t_end = s[gt..].find(&close)? + gt;
    Some((id, strip_tags(&s[gt..t_end])))
}

/// Build a URL path for a section/chapter id (ids are already URL-safe).
fn page_path(id: &str) -> String {
    format!("/r/{}", id.trim_start_matches('_'))
}

fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let end = s[start..].find(close)? + start;
    Some(&s[start..end])
}

/// Strip HTML tags from a short heading, collapsing to plain text.
fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}

/// Escape text for safe inclusion in HTML.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Like `esc`, but leaves `&` alone — for strings such as titles that already
/// contain HTML entities (e.g. "What is Git?" is fine, "&amp;" stays intact).
fn esc_no_amp(s: &str) -> String {
    s.replace('<', "&lt;").replace('>', "&gt;")
}

// ---------------------------------------------------------------------------
// Layout stylesheet (appended after Asciidoctor's default content styles)
// ---------------------------------------------------------------------------

const LAYOUT_CSS: &str = r#"
/* --- Pro Git multi-page reader layout --- */
:root { --sidebar: 19rem; --accent: #b34700; }
html { scroll-behavior: smooth; }
body.reader { margin: 0; background: #fff; }
.layout { display: flex; align-items: flex-start; }
.book-toc {
  position: sticky; top: 0; align-self: flex-start;
  width: var(--sidebar); min-width: var(--sidebar);
  height: 100vh; overflow-y: auto;
  box-sizing: border-box; padding: 1.2rem 1rem 3rem;
  background: #f7f7f8; border-right: 1px solid #e3e3e6;
  font-size: 14px; line-height: 1.4;
}
.book-toc-title {
  display: block; font-size: 1.15rem; font-weight: 700;
  color: #333; text-decoration: none; margin-bottom: .8rem;
}
.book-toc ul { list-style: none; margin: 0; padding: 0; }
.book-toc > ul > li { margin: .15rem 0; }
.book-toc ul ul { margin: .1rem 0 .5rem .2rem; padding-left: .6rem; border-left: 1px solid #e0e0e3; }
.book-toc a { color: #40464f; text-decoration: none; display: block; padding: .18rem .3rem; border-radius: 4px; }
.book-toc a.chap { font-weight: 600; color: #24292f; }
.book-toc a:hover { background: #ececef; color: #000; }
.book-toc a.current { background: var(--accent); color: #fff !important; }
.main {
  flex: 1 1 auto; min-width: 0; box-sizing: border-box;
  max-width: 46rem; margin: 0 auto; padding: 1.5rem 2rem 5rem;
}
.main .content { overflow-wrap: break-word; }
.breadcrumb { font-size: .85rem; text-transform: uppercase; letter-spacing: .04em; margin-bottom: .5rem; }
.breadcrumb a { color: var(--accent); text-decoration: none; }
.pager { display: flex; align-items: center; gap: .5rem; margin: 1rem 0; font-size: .9rem; }
.pager .prev { margin-right: auto; }
.pager .next { margin-left: auto; text-align: right; }
.pager .home { color: #6a737d; text-decoration: none; }
.pager a {
  color: var(--accent); text-decoration: none;
  padding: .35rem .6rem; border: 1px solid #e3e3e6; border-radius: 6px;
  max-width: 45%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
.pager a:hover { background: #faf3ee; border-color: var(--accent); }
.pager .disabled { flex: 0 0 auto; }
.pager.top { border-bottom: 1px solid #eee; padding-bottom: 1rem; }
.pager.bottom { border-top: 1px solid #eee; padding-top: 1rem; }
.main img { max-width: 100%; height: auto; }
.main h2 { margin-top: 0; }
/* Index / landing page */
.index { max-width: 52rem; }
.index .cover { text-align: center; margin: 1rem 0 2rem; }
.index .cover img { max-height: 26rem; width: auto; box-shadow: 0 6px 24px rgba(0,0,0,.18); border-radius: 4px; }
.index .lede { color: #57606a; font-size: 1.05rem; }
.toc-index { list-style: none; padding: 0; }
.toc-index > li { margin: .6rem 0; }
.toc-index a.chap { font-size: 1.1rem; font-weight: 600; color: #24292f; text-decoration: none; }
.toc-index a { color: #40464f; text-decoration: none; }
.toc-index a:hover { color: var(--accent); text-decoration: underline; }
.toc-index ul { list-style: none; padding-left: 1.1rem; margin: .3rem 0 .8rem; }
.toc-index ul li { margin: .12rem 0; }
/* Responsive: stack the sidebar above the content on narrow screens */
@media (max-width: 800px) {
  .layout { flex-direction: column; }
  .book-toc {
    position: static; width: 100%; min-width: 0; height: auto;
    max-height: 40vh; border-right: none; border-bottom: 1px solid #e3e3e6;
  }
  .main { padding: 1rem 1.1rem 4rem; }
}
"#;
