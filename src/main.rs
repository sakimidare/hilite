use clap::Parser;
use highlite::arg_parser::CliArgs;
use highlite::run;

fn main() -> anyhow::Result<()> {
    let cli = CliArgs::parse();
    run(cli)
}
