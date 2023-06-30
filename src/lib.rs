pub(crate) mod workspace;
pub mod build;

use std::path::Path;

/// Make sure that direcetory exists.
fn ensure_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
