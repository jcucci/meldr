//! weavr CLI - Command-line interface for merge conflict resolution
//!
//! This binary provides:
//! - Interactive mode (launches TUI)
//! - Headless mode (applies rules automatically)
//! - File discovery and orchestration

#![forbid(unsafe_code)]

mod cli;
mod discovery;
mod error;
mod headless;
mod tui;

use clap::Parser;

use cli::{Cli, Strategy};
use error::{exit_codes, CliError};

fn run(cli: &Cli) -> Result<i32, CliError> {
    // Mode: List conflicted files
    if cli.list {
        discovery::list_conflicted_files()?;
        return Ok(exit_codes::SUCCESS);
    }

    // Resolve which files to process
    let files = discovery::resolve_files(cli.files.clone())?;

    // Mode: Headless
    if cli.headless {
        let strategy = cli.strategy.unwrap_or(Strategy::Left);

        for path in &files {
            let result = headless::process_file(path, strategy, cli.dedupe)?;
            headless::write_or_print(&result, cli.dry_run)?;
        }

        return Ok(exit_codes::SUCCESS);
    }

    // Mode: Interactive (TUI)
    for path in &files {
        let result = tui::process_file(path)?;

        if let Some(ref content) = result.content {
            std::fs::write(path, content)?;
            println!(
                "{}: {} hunks resolved",
                path.display(),
                result.hunks_resolved
            );
        } else {
            eprintln!(
                "{}: exited with {}/{} hunks unresolved",
                path.display(),
                result.total_hunks - result.hunks_resolved,
                result.total_hunks
            );
        }
    }

    Ok(exit_codes::SUCCESS)
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match run(&cli) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("weavr: {e}");
            e.exit_code()
        }
    };

    std::process::exit(exit_code);
}
