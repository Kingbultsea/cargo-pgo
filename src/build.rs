use std::process::{Child, ChildStdout, Command, Stdio};
use cargo_metadata::{Artifact, Message, MessageIter};
use std::io::{BufReader, Write as _};
use std::collections::HashMap;
use crate::get_default_target;

// 当有id和fmt的write, 可以使用as来兼容
use std::fmt::Write as _;

// Defalut默认值, filtered为空Vec contains_target
#[derive(Debug, Default)]
struct CargoArgs {
    /// 被传递到rustc的参数
    filtered: Vec<String>,
    /// 检测是否存在`--target`
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

/// cargo command spawn
fn cargo_command(
    cargo_cmd: CargoCommand,
    cargo_args: Vec<String>,
    env: HashMap<String, String>,
    release_mode: ReleaseMode,
) -> anyhow::Result<Child> {
    // 过滤`--message-format`  `--release`的warning输出（自动添加）  `--target`标准化为CargoArgs Struct，其余的push进filtered字段
    let parsed_args = parse_cargo_args(cargo_args);

    // `--message-format`
    let mut command = Command::new("cargo");
    command.args(&[
        cargo_cmd.to_str(),
        "--message-format",
        // `json-diagnostic-rendered-ansi` 表示消息以 JSON 格式输出，并带有 ANSI 转义码，用于渲染颜色和格式
        "json-diagnostic-rendered-ansi",
    ]);

    // 将标准输入流与当前进程的标准输入进行关联（继承）
    command.stdin(Stdio::inherit());
    // 将标准输出流设置为管道的作用是将子进程的输出结果通过管道传递给当前进程，以便在当前进程中进行处理或显示
    command.stdout(Stdio::piped());
    command.stderr(Stdio::inherit());

    match release_mode {
        ReleaseMode::AddRelease => {
            command.arg("--release");
        }
        ReleaseMode::NoRelease => {}
    }

    // 如没有指定`--target`，则获取构建目标平台（默认），如--target=x86_64-unknown-linux-gnu
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
            // 自动添加`--release`
            "--release" => {
                log::warn!("`--release`会被自动添加，不需要额外设置");
            }
            // 自动添加`--message-format` 用于指定输出消息的格式
            "--message-format" => {
                log::warn!("`--message-format`会被自动添加，不需要额外设置");
                iterator.next(); // 丢弃值
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
