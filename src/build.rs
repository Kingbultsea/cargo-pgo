use std::process::{Child, ChildStdout, Command, Stdio};
use cargo_metadata::{Artifact, Message, MessageIter};
use std::io::{BufReader, Write as _};
use std::collections::HashMap;

// 当有id和fmt的write, 可以使用as来兼容
use std::fmt::Write as _;

// Defalut默认值, filtered为空Vec contains_target
#[derive(Debug, Default)]
struct CargoArgs {
    filtered: Vec<String>,
    contains_target: bool,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum CargoCommand {
    Build,
    Bench
}

// 是否添加 --release 参数
enum ReleaseMode {
    AddRelease,
    NoRelease,
}

pub struct RunningCargo {
    // 由Command创建
    child: Child,
    // 获取child输出的信息
    message_iter: MessageIter<BufReader<ChildStdout>>,
}

impl CargoCommand {
    pub fn to_str(&self) -> &str {
        match self {
            CargoCommand::Build => "build",
            CargoCommand::Bench => "bench",
        }
    }
}

// 运行cargo
// todo 为什么不用Vec<&str>呢
pub fn cargo_command_with_flags(
    command: CargoCommand,
    flags: &str,
    cargo_args: Vec<String>,
) -> anyhow::Result<RunningCargo> {
    let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
    write!(&mut rustflags, " {}", flags).unwrap();

    let mut env: HashMap<String, String> = HashMap::default();
    env.insert("RUSTFLAGS".to_string(), rustflags);
    let release_mode = match command {
        CargoCommand::Bench => ReleaseMode::NoRelease,
        _ => ReleaseMode::AddRelease,
    };

    println!("cargo_command_with_flags参数 {:?}", env);

    let mut child = cargo_command(command, cargo_args, env, release_mode)?;
    let stdout = child.stdout.take().unwrap();
    Ok(RunningCargo {
        child,
        message_iter: Message::parse_stream(BufReader::new(stdout)),
    })
}

/// Spawn `cargo` command in release mode with the provided env variables and Cargo arguments.
fn cargo_command(
    cargo_cmd: CargoCommand,
    cargo_args: Vec<String>,
    env: HashMap<String, String>,
    release_mode: ReleaseMode,
) -> anyhow::Result<Child> {
    let parsed_args = parse_cargo_args(cargo_args);

    let mut command = Command::new("cargo");
    command.args(&[
        cargo_cmd.to_str(),
        "--message-format",
        "json-diagnostic-rendered-ansi",
    ]);

    // 将子进程的标准输入与当前进程的标准输入相同，即子进程继承了当前进程的标准输入。这意味着子进程可以从标准输入读取数据
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::inherit());

    match release_mode {
        ReleaseMode::AddRelease => {
            command.arg("--release");
        }
        ReleaseMode::NoRelease => {}
    }

    // --target is passed to avoid instrumenting build scripts
    // See https://doc.rust-lang.org/rustc/profile-guided-optimization.html#a-complete-cargo-workflow
    if !parsed_args.contains_target {
        let default_target = get_default_target().map_err(|error| {
            anyhow::anyhow!(
                "Unable to find default target triple for your platform: {:?}",
                error
            )
        })?;
        command.args(&["--target", &default_target]);
    }

    for arg in parsed_args.filtered {
        command.arg(arg);
    }
    for (key, value) in env {
        command.env(key, value);
    }
    log::debug!("Executing cargo command: {:?}", command);
    Ok(command.spawn()?)
}

fn parse_cargo_args(cargo_args: Vec<String>) -> CargoArgs {
    let mut args = CargoArgs::default();

    let mut iterator = cargo_args.into_iter();
    while let Some(arg) = iterator.next() {
        match arg.as_str() {
            // Skip `--release`, we will pass it by ourselves.
            "--release" => {
                log::warn!("Do not pass `--release` manually, it will be added automatically by `cargo-pgo`");
            }
            // Skip `--message-format`, we need it to be JSON.
            "--message-format" => {
                log::warn!("Do not pass `--message-format` manually, it will be added automatically by `cargo-pgo`");
                iterator.next(); // skip flag value
            }
            "--target" => {
                args.contains_target = true;
                args.filtered.push(arg);
            }
            _ => args.filtered.push(arg),
        }
    }
    args
}
