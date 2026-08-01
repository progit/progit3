//! A tiny, dependency-free HTTP server for reading the book locally as a
//! multi-page site — one page per section, like <https://git-scm.com/book>.
//!
//! Generated pages (the reader, its stylesheet, the index) are held in memory
//! by [`Site`]; anything else — images, the cover — is served from disk. Only
//! `GET`/`HEAD` are supported and on-disk paths are sandboxed to the project
//! root.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::thread;

use crate::context::Context;
use crate::error::{BuildError, Result};
use crate::site::Site;
use crate::tasks;
use crate::ui;

pub struct ServeOptions {
    pub port: u16,
    pub build_first: bool,
    pub open: bool,
}

/// Shared state handed to every connection thread.
struct Server {
    root: PathBuf,
    site: Site,
}

/// Build the multi-page reader and serve it until interrupted.
pub fn serve(ctx: &Context, opts: ServeOptions) -> Result<()> {
    // Obtain the full single-file HTML, then split it into pages. By default we
    // render fresh to a scratch file (non-data-uri, so image references stay as
    // `images/…` paths served from disk — far lighter than embedding every
    // image on every page). With --no-build we reuse an existing progit.html
    // instead, avoiding the Asciidoctor round-trip entirely.
    let full = if opts.build_first {
        let scratch = std::env::temp_dir().join("progit-serve-full.html");
        tasks::ensure_contributors(ctx)?;
        ui::step("Building the book for local reading …");
        tasks::build_html_to(ctx, &scratch, false)?;
        let html = fs::read_to_string(&scratch).map_err(|e| {
            BuildError::new(format!(
                "could not read generated HTML at {}: {e}",
                scratch.display()
            ))
        })?;
        let _ = fs::remove_file(&scratch);
        html
    } else {
        let existing = ctx.path("progit.html");
        if !existing.exists() {
            return Err(BuildError::new(
                "progit.html does not exist yet; run without --no-build, or run `progit html` first.",
            ));
        }
        ui::info("Reusing existing progit.html (--no-build) …");
        fs::read_to_string(&existing)?
    };

    ui::info("Splitting into per-section pages …");
    let site = Site::build(&full)?;
    ui::done(&format!("{} reader pages ready", site.page_count));

    let listener = TcpListener::bind(("127.0.0.1", opts.port)).map_err(|e| {
        BuildError::new(format!(
            "could not bind 127.0.0.1:{}: {e}\nTry a different --port.",
            opts.port
        ))
    })?;
    let port = listener.local_addr().map(|a| a.port()).unwrap_or(opts.port);
    let url = format!("http://127.0.0.1:{port}/");

    ui::step(&format!("Serving the book at {url}"));
    ui::info("Press Ctrl-C to stop.");

    if opts.open {
        open_browser(&url);
    }

    let server = Arc::new(Server {
        root: ctx.root.clone(),
        site,
    });
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server = Arc::clone(&server);
                thread::spawn(move || {
                    if let Err(e) = handle(stream, &server) {
                        // A broken pipe just means the browser moved on.
                        if e.kind() != std::io::ErrorKind::BrokenPipe {
                            ui::warn(&format!("connection error: {e}"));
                        }
                    }
                });
            }
            Err(e) => ui::warn(&format!("failed to accept connection: {e}")),
        }
    }
    Ok(())
}

fn handle(stream: TcpStream, server: &Server) -> std::io::Result<()> {
    let peer_read = stream.try_clone()?;
    let mut reader = BufReader::new(peer_read);
    let mut writer = stream;

    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }

    // Drain the remaining headers so the client is happy.
    let mut header = String::new();
    while reader.read_line(&mut header)? > 0 {
        if header == "\r\n" || header == "\n" {
            break;
        }
        header.clear();
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_target = parts.next().unwrap_or("/");

    if method != "GET" && method != "HEAD" {
        return respond(&mut writer, 405, "Method Not Allowed", b"405 Method Not Allowed", "text/plain; charset=utf-8", method == "HEAD");
    }

    let target = raw_target.split(['?', '#']).next().unwrap_or("/");
    let head_only = method == "HEAD";

    // Generated reader pages first, then static assets from disk.
    if let Some((ctype, body)) = server.site.get(target) {
        return respond(&mut writer, 200, "OK", body, ctype, head_only);
    }

    match resolve(&server.root, target) {
        Some(path) => serve_file(&mut writer, &path, head_only),
        None => respond(
            &mut writer,
            404,
            "Not Found",
            b"404 Not Found",
            "text/plain; charset=utf-8",
            head_only,
        ),
    }
}

/// Map a URL path onto a file inside `root`, rejecting anything that escapes it.
fn resolve(root: &Path, target: &str) -> Option<PathBuf> {
    let decoded = percent_decode(target.trim_start_matches('/'));
    let mut path = root.to_path_buf();
    for comp in Path::new(&decoded).components() {
        match comp {
            Component::Normal(seg) => path.push(seg),
            // Refuse `..`, absolute roots and drive prefixes.
            _ => return None,
        }
    }
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}

fn serve_file(writer: &mut TcpStream, path: &Path, head_only: bool) -> std::io::Result<()> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            return respond(
                writer,
                404,
                "Not Found",
                b"404 Not Found",
                "text/plain; charset=utf-8",
                head_only,
            )
        }
    };
    let mut body = Vec::new();
    file.read_to_end(&mut body)?;
    let ctype = content_type(path);
    respond(writer, 200, "OK", &body, ctype, head_only)
}

fn respond(
    writer: &mut TcpStream,
    code: u16,
    status: &str,
    body: &[u8],
    content_type: &str,
    head_only: bool,
) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {code} {status}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n",
        body.len()
    );
    writer.write_all(header.as_bytes())?;
    if !head_only {
        writer.write_all(body)?;
    }
    writer.flush()
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "pdf" => "application/pdf",
        "epub" => "application/epub+zip",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Decode `%XX` escapes and `+` in a URL path segment.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push((hi * 16 + lo) as u8);
                    i += 3;
                    continue;
                }
                out.push(b'%');
                i += 1;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Best-effort attempt to open the system browser; failure is non-fatal.
fn open_browser(url: &str) {
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "explorer"
    } else {
        "xdg-open"
    };
    if std::process::Command::new(opener).arg(url).spawn().is_err() {
        ui::info("(could not open a browser automatically — open the URL above)");
    }
}
