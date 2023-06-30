use cargo_pgoe::build::CargoCommand;
use cargo_pgoe::workspace::CargoContext;

#[derive(clap::Parser, Debug)]
#[clap(trailing_var_arg(true))]
pub struct PgoInstrumentArgs {
    /// Cargo command that will be used for PGO-instrumented compilation.
    #[clap(value_enum, default_value = "build")]
    command: CargoCommand,
}

pub fn pgo_instrument(ctx: CargoContext, args: PgoInstrumentArgs) {

}