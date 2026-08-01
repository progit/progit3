//! Hand-rolled argument parsing (kept dependency-free on purpose).

use crate::error::{BuildError, Result};

pub const USAGE: &str = "\
progit — build system for the Pro Git book

USAGE:
    progit <command> [options]

COMMANDS:
    html            Build the single-file HTML book (progit.html)
    pdf             Build the PDF book (progit.pdf)
    epub            Build the EPUB book (progit.epub)
    mobi            Build the Mobi/KF8 book (progit.mobi)
    all             Build every format above
    serve           Build the HTML and serve it on a local web server
    contributors    Regenerate book/contributors.txt if needed
    clean           Remove all generated files
    help            Show this help

GLOBAL OPTIONS:
    --no-bundle     Invoke asciidoctor tools directly instead of `bundle exec`
    -h, --help      Show this help
    -V, --version   Show version

SERVE OPTIONS:
    -p, --port <N>  Port to listen on (default 8080)
    --open          Open the book in your browser once it is ready
    --no-build      Serve the existing progit.html without rebuilding

EXAMPLES:
    progit serve --port 3000 --open
    progit pdf
    progit all
";

/// A parsed command line.
pub enum Command {
    Build(Vec<crate::tasks::Format>),
    Serve {
        port: u16,
        open: bool,
        build_first: bool,
    },
    Contributors,
    Clean,
    Help,
    Version,
}

pub struct Cli {
    pub command: Command,
    pub no_bundle: bool,
}

pub fn parse(args: &[String]) -> Result<Cli> {
    use crate::tasks::Format;

    let mut positional: Vec<String> = Vec::new();
    let mut no_bundle = false;
    let mut open = false;
    let mut build_first = true;
    let mut port: u16 = 8080;
    let mut want_help = false;
    let mut want_version = false;

    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => want_help = true,
            "-V" | "--version" => want_version = true,
            "--no-bundle" => no_bundle = true,
            "--open" => open = true,
            "--no-build" => build_first = false,
            "-p" | "--port" => {
                let value = it
                    .next()
                    .ok_or_else(|| BuildError::new("--port requires a number"))?;
                port = value
                    .parse()
                    .map_err(|_| BuildError::new(format!("invalid port: {value}")))?;
            }
            other if other.starts_with("--port=") => {
                let value = &other["--port=".len()..];
                port = value
                    .parse()
                    .map_err(|_| BuildError::new(format!("invalid port: {value}")))?;
            }
            other if other.starts_with('-') => {
                return Err(BuildError::new(format!("unknown option: {other}")));
            }
            other => positional.push(other.to_string()),
        }
    }

    if want_version {
        return Ok(Cli {
            command: Command::Version,
            no_bundle,
        });
    }
    if want_help || positional.is_empty() {
        return Ok(Cli {
            command: Command::Help,
            no_bundle,
        });
    }

    let command = match positional[0].as_str() {
        "html" => Command::Build(vec![Format::Html]),
        "pdf" => Command::Build(vec![Format::Pdf]),
        "epub" => Command::Build(vec![Format::Epub]),
        "mobi" => Command::Build(vec![Format::Mobi]),
        "all" | "build" => Command::Build(Format::all().to_vec()),
        "serve" => Command::Serve {
            port,
            open,
            build_first,
        },
        "contributors" => Command::Contributors,
        "clean" => Command::Clean,
        "help" => Command::Help,
        other => {
            return Err(BuildError::new(format!(
                "unknown command: {other}\nRun `progit help` for usage."
            )))
        }
    };

    Ok(Cli { command, no_bundle })
}
