use crate::build::{CargoCommand, cargo_command_with_flags};
use crate::clear_directory;
use crate::workspace::CargoContext;
use crate::cli::cli_format_path;

#[derive(clap::Parser, Debug)]
#[clap(trailing_var_arg(true))]
pub struct PgoInstrumentArgs {
    /// Cargo command that will be used for PGO-instrumented compilation.
    #[clap(value_enum, default_value = "build")]
    command: CargoCommand,

    /// Do not remove profiles that were gathered during previous runs.
    /// takes_value = false 表示该选项不接受值，它是一个开关选项
    #[clap(long, takes_value = false)]
    keep_profiles: bool,
}

pub fn pgo_instrument(ctx: CargoContext, args: PgoInstrumentArgs) -> anyhow::Result<()> {
    let pgo_dir = ctx.get_pgo_directory()?;

    println!("{:?}", args);

    if !args.keep_profiles {
        log::info!("PGO profile directory will be cleared.");
        clear_directory(&pgo_dir)?;
    }

    // 创建pgo文件夹
    log::info!(
        "PGO profiles will be stored into {}.",
        cli_format_path(pgo_dir.display())
    );

    // pgo位置参数
    let flags = format!("-Cprofile-generate={}", pgo_dir.display());

    let mut cargo = cargo_command_with_flags(args.command, &flags, args.cargo_args)?;

    Ok(())
}