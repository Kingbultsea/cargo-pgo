mod check;
mod pgo;

use clap::Parser;
use check::environment_info;
use pgo::instrument::{PgoInstrumentArgs, pgo_instrument};

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
    /// Execute a `cargo` command to create PGO-instrumented artifact(s).
    /// After the artifacts are executed, they will produce profiles that can be later used in the
    /// `optimize` step.
    Instrument(PgoInstrumentArgs),
}

fn run() {
    let args = Args::parse();

    let Args::Pgoe(args) = args;

    let _ = match args {
        Subcommand::Info => environment_info(),
        Subcommand::Instrument(_) => todo!(),
    };

    println!("打印结构体：{:?}", args);
}

fn main() {
    run();
}
