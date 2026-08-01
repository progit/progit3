//! Build context: project paths, version/date attributes and the logic for
//! invoking the Asciidoctor toolchain (directly or through Bundler).

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{BuildError, Result};

/// The master AsciiDoc document that stitches the whole book together.
pub const MASTER_DOC: &str = "progit.asc";

/// Everything a build task needs to know about where it runs and how it
/// should shell out to the Ruby tooling.
pub struct Context {
    /// Absolute path to the repository root (the directory holding `progit.asc`).
    pub root: PathBuf,
    /// `revnumber` attribute passed to Asciidoctor (e.g. `2.1.4`).
    pub version: String,
    /// `revdate` attribute passed to Asciidoctor (today, `YYYY-MM-DD`).
    pub date: String,
    /// When true, run the gem executables directly instead of `bundle exec`.
    pub no_bundle: bool,
}

impl Context {
    /// Discover the project root (searching upward for `progit.asc`), read the
    /// version from git tags and capture today's date.
    pub fn discover(no_bundle: bool) -> Result<Context> {
        let root = find_root()?;
        let version = detect_version(&root);
        let date = today();
        Ok(Context {
            root,
            version,
            date,
            no_bundle,
        })
    }

    /// Absolute path to a file relative to the project root.
    pub fn path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    /// The `--attribute revnumber=… --attribute revdate=…` arguments shared by
    /// every Asciidoctor invocation.
    pub fn doc_attrs(&self) -> Vec<String> {
        vec![
            "--attribute".into(),
            format!("revnumber={}", self.version),
            "--attribute".into(),
            format!("revdate={}", self.date),
        ]
    }

    /// Build a `Command` for one of the Asciidoctor gem executables, wiring it
    /// through Bundler when a `Gemfile` is present (unless disabled).
    pub fn tool(&self, name: &str) -> Command {
        let use_bundle = !self.no_bundle
            && self.path("Gemfile").exists()
            && which("bundle").is_some();

        let mut cmd = if use_bundle {
            let mut c = Command::new("bundle");
            c.arg("exec").arg(name);
            c
        } else {
            Command::new(name)
        };
        cmd.current_dir(&self.root);
        cmd
    }
}

/// Locate the project root by walking up from the current directory until a
/// `progit.asc` is found.
fn find_root() -> Result<PathBuf> {
    let start = env::current_dir().map_err(|e| BuildError::new(format!("cannot read cwd: {e}")))?;
    let mut dir = start.as_path();
    loop {
        if dir.join(MASTER_DOC).is_file() {
            return Ok(dir.to_path_buf());
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => {
                return Err(BuildError::new(format!(
                    "could not find '{MASTER_DOC}' in {} or any parent directory.\n\
                     Run this from inside the Pro Git repository.",
                    start.display()
                )))
            }
        }
    }
}

/// Mirror the Rakefile's version logic: take the latest tag `x.y.z` and bump
/// the patch component; fall back to `0` when the repo has no tags.
fn detect_version(root: &Path) -> String {
    let described = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .current_dir(root)
        .output();

    let raw = match described {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => String::new(),
    };

    if raw.is_empty() {
        return "0".to_string();
    }

    let parts: Vec<&str> = raw.split('.').collect();
    if parts.len() >= 3 {
        let patch = leading_int(parts[2]) + 1;
        format!("{}.{}.{}", parts[0], parts[1], patch)
    } else {
        raw
    }
}

/// Parse the leading integer of a string (`"4rc1"` -> `4`), like Ruby's `to_i`.
fn leading_int(s: &str) -> u64 {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(0)
}

/// Today's date as `YYYY-MM-DD`. Shells out to `date` to stay dependency-free.
fn today() -> String {
    Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Minimal `which`: is an executable of this name on `PATH`?
pub fn which(name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    p.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(p: &Path) -> bool {
    p.is_file()
}

/// Run a prepared command to completion, streaming its output, and turn a
/// non-zero exit (or a missing executable) into a friendly error.
pub fn run(mut cmd: Command, tool_hint: &str) -> Result<()> {
    let program = cmd.get_program().to_string_lossy().to_string();
    let status = cmd.status().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            BuildError::new(format!(
                "'{program}' was not found.\n{}",
                install_hint(tool_hint)
            ))
        } else {
            BuildError::new(format!("failed to launch '{program}': {e}"))
        }
    })?;

    if !status.success() {
        let code = status
            .code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "signal".to_string());
        return Err(BuildError::new(format!(
            "'{program} {}' exited with status {code}",
            display_args(&cmd)
        )));
    }
    Ok(())
}

fn display_args(cmd: &Command) -> String {
    cmd.get_args()
        .map(OsStr::to_string_lossy)
        .collect::<Vec<_>>()
        .join(" ")
}

fn install_hint(tool: &str) -> String {
    format!(
        "The Asciidoctor toolchain (needed for '{tool}') is not installed.\n\
         Install the Ruby dependencies first:\n    bundle install\n\
         If the gems are installed globally rather than via Bundler, re-run with --no-bundle."
    )
}
