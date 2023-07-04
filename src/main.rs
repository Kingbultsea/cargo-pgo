mod check;

use clap::Parser;
use check::environment_info;
use cargo_pgoe::get_cargo_ctx;
use cargo_pgoe::pgo::instrument::{PgoInstrumentArgs, pgo_instrument};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
#[clap(bin_name("cargo"))]
#[clap(disable_help_subcommand(true))]
enum Args {
    #[clap(subcommand)]
    Pgoe(Subcommand),
}

#[derive(clap::Subcommand, Debug)]
#[clap(setting(clap::AppSettings::DeriveDisplayOrder))]
enum Subcommand {
    Info,
    Instrument(PgoInstrumentArgs),
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let ctx = get_cargo_ctx()?;

    let Args::Pgoe(args) = args;

    match args {
        Subcommand::Info => environment_info(),
        Subcommand::Instrument(args) => pgo_instrument(ctx, args),
    }
}

fn main() {
    let _ = run();

    use std::process::{Command, Stdio};
    use std::io::{self, Write};
    
    let output = Command::new("rev")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    
    print!("You piped in the reverse of: ");
    io::stdout().write_all(&output.stdout).unwrap();
}
