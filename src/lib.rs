pub mod pgo;
pub mod build;
pub mod cli;
pub mod workspace;

pub use workspace::get_cargo_ctx;
use std::path::Path;
use anyhow::anyhow;

// 操作系统原生字符串，保证不同平台的兼容性&正确性
// 从不同的字符串表示形式创建 OsStr 对象，例如从 UTF-8 字节序列、UTF-16 字节序列或操作系统原生编码的字节序列。
// 将 OsStr 转换为其他字符串类型，如 String 或 &str。
// 比较 OsStr 对象，例如检查两个路径是否相等。
// 与操作系统进行交互，例如将 OsStr 传递给操作系统原生函数或调用操作系统相关的 API。
use std::ffi::OsStr;
use std::process::{Command, ExitStatus};

pub const GREETING: &'static str = "Hallo, Rust library here!";

#[derive(Debug)]
struct Utf8Output {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

/// Make sure that direcetory exists.
fn ensure_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Clears all files from the directory, and recreates it.
fn clear_directory(path: &Path) -> std::io::Result<()> {
    std::fs::remove_dir_all(path)?;
    ensure_directory(path)
}

/// 获取当前编译器所处环境的平台，用于默认目标的构建
pub fn get_default_target() -> anyhow::Result<String> {
    get_rustc_info("host: ")
}

fn get_rustc_info(field: &str) -> anyhow::Result<String> {
    // Query rustc for defaults.
    let output = run_command("rustc", &["-vV"])?;

    // Parse the field from stdout.
    let host = output
        .stdout
        .lines()
        .find(|l| l.starts_with(field))
        .map(|l| l[field.len()..].trim())
        .ok_or_else(|| anyhow!("Failed to parse field {} from rustc output.", field))?
        .to_owned();
    Ok(host)
}

fn run_command<S: AsRef<OsStr>, Str: AsRef<OsStr>>(
    program: S,
    args: &[Str],
) -> anyhow::Result<Utf8Output> {
    let mut cmd = Command::new(program);
    for arg in args {
        cmd.arg(arg);
    }
    log::debug!("Running command {:?}", cmd);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let output = cmd.output()?;
    Ok(Utf8Output {
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8(output.stderr)?,
        status: output.status,
    })
}
