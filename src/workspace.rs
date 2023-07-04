use crate::ensure_directory;
use std::path::{Path, PathBuf};

pub struct CargoContext {
    target_directory: PathBuf,
}

impl CargoContext {
    pub fn get_pgo_directory(&self) -> anyhow::Result<PathBuf> {
        self.get_target_directory(Path::new("pgo-profiles"))
    }

    fn get_target_directory(&self, path: &Path) -> anyhow::Result<PathBuf> {
        let directory = self.target_directory.join(path);
        ensure_directory(&directory)?;
        Ok(directory)
    }
}

/// Finds Cargo metadata from the current directory.
pub fn get_cargo_ctx() -> anyhow::Result<CargoContext> {
    log::info!("正在执行MetadataCommand，获取项目信息，该过程可能会比较久。");

    let cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd
        .exec()
        .map_err(|error| anyhow::anyhow!("Cannot get cargo metadata: {:?}", error))?;

    // target构建的path
    // 如/Users/dorho/Desktop/rust-work-place/cargo-pgoe/target
    let buf_path = metadata.target_directory.into_std_path_buf();

    Ok(CargoContext {
        target_directory: buf_path,
    })
}
