//! TUI mode implementation.

use std::path::Path;

use weavr_core::MergeSession;
use weavr_tui::App;

use crate::error::CliError;

/// Result of TUI processing for a single file.
pub struct TuiResult {
    /// The resolved content (if fully resolved and saved).
    pub content: Option<String>,
    /// Number of hunks that were resolved.
    pub hunks_resolved: usize,
    /// Total number of hunks in the file.
    pub total_hunks: usize,
}

/// Runs the TUI for a single file.
///
/// Returns the resolution result after the user quits the TUI.
pub fn process_file(path: &Path) -> Result<TuiResult, CliError> {
    let content = std::fs::read_to_string(path)?;
    let session = MergeSession::from_conflicted(&content, path.to_path_buf())?;

    // Handle files without conflicts (already clean)
    if session.hunks().is_empty() {
        return Ok(TuiResult {
            content: Some(content),
            hunks_resolved: 0,
            total_hunks: 0,
        });
    }

    let total_hunks = session.hunks().len();

    // Create and configure App
    let mut app = App::new();
    app.set_session(session);

    // Run TUI event loop
    weavr_tui::run(&mut app)?;

    // Extract session and check resolution state
    let session = app
        .take_session()
        .ok_or_else(|| std::io::Error::other("merge session unexpectedly missing after TUI run"))?;
    let resolved_count = session
        .hunks()
        .iter()
        .filter(|h| matches!(h.state, weavr_core::HunkState::Resolved(_)))
        .count();

    if session.is_fully_resolved() {
        // Complete the lifecycle to get the merged content
        let mut session = session;
        session.apply()?;
        session.validate()?;
        let result = session.complete()?;

        Ok(TuiResult {
            content: Some(result.content),
            hunks_resolved: result.summary.resolved_hunks,
            total_hunks,
        })
    } else {
        // User quit without resolving all hunks
        Ok(TuiResult {
            content: None,
            hunks_resolved: resolved_count,
            total_hunks,
        })
    }
}
