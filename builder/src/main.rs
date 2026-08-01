//! `progit` — a self-contained build system for the Pro Git book.
//!
//! It orchestrates the Asciidoctor toolchain to produce HTML, PDF, EPUB and
//! Mobi editions, and can serve the HTML edition over a local web server for
//! comfortable offline reading. The crate has no third-party dependencies.

mod cli;
mod context;
mod error;
mod serve;
mod site;
mod tasks;
mod ui;

use cli::Command;
use context::Context;
use error::Result;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        ui::error(&e.to_string());
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<()> {
    let parsed = cli::parse(args)?;

    match parsed.command {
        Command::Help => {
            print!("{}", cli::USAGE);
            Ok(())
        }
        Command::Version => {
            println!("progit {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Build(formats) => {
            let ctx = Context::discover(parsed.no_bundle)?;
            banner(&ctx);
            for format in formats {
                tasks::build(&ctx, format)?;
            }
            Ok(())
        }
        Command::Serve {
            port,
            open,
            build_first,
        } => {
            let ctx = Context::discover(parsed.no_bundle)?;
            banner(&ctx);
            serve::serve(
                &ctx,
                serve::ServeOptions {
                    port,
                    open,
                    build_first,
                },
            )
        }
        Command::Contributors => {
            let ctx = Context::discover(parsed.no_bundle)?;
            tasks::ensure_contributors(&ctx)?;
            ui::done("contributors list is up to date");
            Ok(())
        }
        Command::Clean => {
            let ctx = Context::discover(parsed.no_bundle)?;
            tasks::clean(&ctx)
        }
    }
}

fn banner(ctx: &Context) {
    ui::info(&format!(
        "Pro Git — version {}, {}",
        ctx.version, ctx.date
    ));
}
