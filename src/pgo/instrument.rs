use crate::build::{cargo_command_with_flags, CargoCommand, get_artifact_kind, handle_metadata_message};
use crate::clear_directory;
use crate::cli::cli_format_path;
use crate::workspace::CargoContext;

use colored::Colorize;
use cargo_metadata::Message;

#[derive(clap::Parser, Debug)]
#[clap(trailing_var_arg(true))]
pub struct PgoInstrumentArgs {
    /// Cargo command that will be used for PGO-instrumented compilation.
    #[clap(value_enum, default_value = "build")]
    command: CargoCommand,

    /// takes_value = false 表示该选项不接受值，它是一个开关选项
    #[clap(long, takes_value = false)]
    keep_profiles: bool,

    cargo_args: Vec<String>,
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

    // pgo文件位置
    let flags = format!("-Cprofile-generate={}", pgo_dir.display());

    let mut cargo = cargo_command_with_flags(args.command, &flags, args.cargo_args)?;

    for message in cargo.message() {
        let message = message?;
        match message {
            // 编译器生成的目标文件或二进制可执行文件的元数据信息 用于输出在控制台上，显示构建信息
            Message::CompilerArtifact(artifact) => {
                if let Some(ref executable) = artifact.executable {
                    if let CargoCommand::Build = args.command {
                        log::info!(
                            "PGO构建成功： {} {}",
                            get_artifact_kind(&artifact).yellow(),
                            artifact.target.name.blue()
                        );
                        log::info!(
                            "Now run {} on your workload.\nFor more precise profiles, run \
it with the following environment variable: {}.",
                            cli_format_path(&executable),
                            format!(
                                "LLVM_PROFILE_FILE={}/{}_%m_%p.profraw",
                                pgo_dir.display(),
                                artifact.target.name
                            )
                            .blue()
                        );
                    }
                }
            }
            Message::BuildFinished(res) => {}
            _ => handle_metadata_message(message),
        }
    }

    Ok(())
}
