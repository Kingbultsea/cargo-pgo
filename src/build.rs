#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum CargoCommand {
    Build,
}

impl CargoCommand {
    pub fn to_str(&self) -> &str {
        match self {
            CargoCommand::Build => "build"
        }
    }
}
