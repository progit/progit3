//! The individual build tasks: contributors list, the four output formats and
//! cleanup. Each mirrors the behaviour of the historical Rakefile.

use std::fs;
use std::process::Command;

use crate::context::{run, Context};
use crate::error::{BuildError, Result};
use crate::ui;

/// One selectable output format.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Html,
    Pdf,
    Epub,
    Mobi,
}

impl Format {
    pub fn all() -> [Format; 4] {
        [Format::Html, Format::Pdf, Format::Epub, Format::Mobi]
    }

    pub fn label(self) -> &'static str {
        match self {
            Format::Html => "HTML",
            Format::Pdf => "PDF",
            Format::Epub => "EPUB",
            Format::Mobi => "Mobi",
        }
    }

    /// The file the format produces at the project root.
    pub fn output(self) -> &'static str {
        match self {
            Format::Html => "progit.html",
            Format::Pdf => "progit.pdf",
            Format::Epub => "progit.epub",
            Format::Mobi => "progit.mobi",
        }
    }
}

/// Ensure `book/contributors.txt` exists and matches the current `HEAD`.
///
/// The file is included by `book/contributors.asc`; its first line embeds the
/// commit it was generated from so a stale list can be detected and rebuilt.
pub fn ensure_contributors(ctx: &Context) -> Result<()> {
    let target = ctx.path("book/contributors.txt");
    let head = git_short_head(ctx)?;

    if target.exists() {
        let first_line = fs::read_to_string(&target)
            .ok()
            .and_then(|c| c.lines().next().map(str::to_string))
            .unwrap_or_default();
        // The header reads "Contributors as of <shorthash>:"; the file is fresh
        // when that hash matches the current HEAD.
        if first_line.contains(&head) {
            return Ok(());
        }
        ui::info("Contributors list is stale, regenerating…");
        let _ = fs::remove_file(&target);
    } else {
        ui::info("Generating contributors list…");
    }

    generate_contributors(ctx, &head)
}

fn generate_contributors(ctx: &Context, head: &str) -> Result<()> {
    // Names contributed to the book, minus the primary authors and bots.
    let shortlog = Command::new("git")
        .args(["shortlog", "-s", "HEAD"])
        .current_dir(&ctx.root)
        .output()?;
    if !shortlog.status.success() {
        return Err(BuildError::new("`git shortlog` failed generating contributors"));
    }

    let mut names: Vec<String> = String::from_utf8_lossy(&shortlog.stdout)
        .lines()
        .filter_map(|line| line.split_once('\t').map(|(_, name)| name.trim().to_string()))
        .filter(|name| {
            !(name.contains("Straub") || name.contains("Chacon") || name.contains("dependabot"))
        })
        .collect();
    names.sort();
    names.dedup();

    let mut body = format!("Contributors as of {head}:\n\n");
    body.push_str(&names.join("\n"));
    body.push('\n');

    fs::write(ctx.path("book/contributors.txt"), body)?;
    Ok(())
}

fn git_short_head(ctx: &Context) -> Result<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(&ctx.root)
        .output()?;
    if !out.status.success() {
        return Err(BuildError::new("`git rev-parse --short HEAD` failed"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Build a single format, returning the path of the produced file.
pub fn build(ctx: &Context, format: Format) -> Result<()> {
    ensure_contributors(ctx)?;
    ui::step(&format!("Building {} …", format.label()));
    match format {
        Format::Html => build_html(ctx, true)?,
        Format::Pdf => build_pdf(ctx)?,
        Format::Epub => build_epub(ctx)?,
        Format::Mobi => build_mobi(ctx)?,
    }
    ui::done(&format!("{} output at {}", format.label(), format.output()));
    Ok(())
}

/// Build the single-file HTML at the default location (`progit.html`).
/// `data_uri` embeds images/CSS for a fully self-contained document.
pub fn build_html(ctx: &Context, data_uri: bool) -> Result<()> {
    build_html_to(ctx, &ctx.path("progit.html"), data_uri)
}

/// Build the single-file HTML to a specific output path. Used by `serve`, which
/// renders to a scratch file it then splits into a multi-page site.
pub fn build_html_to(ctx: &Context, out: &std::path::Path, data_uri: bool) -> Result<()> {
    let mut cmd = ctx.tool("asciidoctor");
    cmd.args(ctx.doc_attrs());
    if data_uri {
        cmd.args(["-a", "data-uri"]);
    }
    cmd.arg("-o").arg(out);
    cmd.arg(crate::context::MASTER_DOC);
    run(cmd, "asciidoctor")
}

fn build_pdf(ctx: &Context) -> Result<()> {
    ui::info("(the PDF takes a while)");
    let mut cmd = ctx.tool("asciidoctor-pdf");
    cmd.args(ctx.doc_attrs());
    cmd.args([
        "-a",
        "pdf-theme=theme/pdf/progit-theme.yml",
        "-a",
        "pdf-fontsdir=theme/pdf/fonts;GEM_FONTS_DIR",
    ]);
    cmd.arg(crate::context::MASTER_DOC);
    run(cmd, "asciidoctor-pdf")
}

fn build_epub(ctx: &Context) -> Result<()> {
    let mut cmd = ctx.tool("asciidoctor-epub3");
    cmd.args(ctx.doc_attrs());
    cmd.arg(crate::context::MASTER_DOC);
    run(cmd, "asciidoctor-epub3")
}

fn build_mobi(ctx: &Context) -> Result<()> {
    // KF8/Mobi is produced by asciidoctor-epub3 in `kf8` mode.
    let mut cmd = ctx.tool("asciidoctor-epub3");
    cmd.args(ctx.doc_attrs());
    cmd.args(["-a", "ebook-format=kf8"]);
    cmd.arg(crate::context::MASTER_DOC);
    run(cmd, "asciidoctor-epub3")
}

/// Remove all generated artifacts.
pub fn clean(ctx: &Context) -> Result<()> {
    let artifacts = [
        "book/contributors.txt",
        "progit.html",
        "progit.pdf",
        "progit.pdfmarks",
        "progit.epub",
        "progit-kf8.epub",
        "progit.fb2.zip",
        "progit.mobi",
    ];
    let mut removed = 0;
    for rel in artifacts {
        let path = ctx.path(rel);
        if path.exists() {
            fs::remove_file(&path)?;
            ui::info(&format!("removed {rel}"));
            removed += 1;
        }
    }
    if removed == 0 {
        ui::info("nothing to clean");
    }
    Ok(())
}
