pub mod pgo;
pub mod build;
pub mod cli;
pub mod workspace;

pub use workspace::get_cargo_ctx;
use std::path::Path;

pub const GREETING: &'static str = "Hallo, Rust library here!";

/// Make sure that direcetory exists.
fn ensure_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Clears all files from the directory, and recreates it.
fn clear_directory(path: &Path) -> std::io::Result<()> {
    std::fs::remove_dir_all(path)?;
    ensure_directory(path)
}
