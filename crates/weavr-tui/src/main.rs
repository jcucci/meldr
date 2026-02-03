//! Development entry point for weavr-tui.
//!
//! This binary is for development and testing purposes.
//! Production use will be through weavr-cli.

use std::io;

use weavr_tui::App;

fn main() -> io::Result<()> {
    let mut app = App::new();
    weavr_tui::run(&mut app)
}
