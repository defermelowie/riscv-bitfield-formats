use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

mod csr;
use csr::to_csr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What do you want to do?
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print CSR in a human readable format
    Print {
        /// CSR
        name: String,

        /// Value
        #[clap(value_parser=maybe_hex::<u64>)]
        value: u64,
    },
    /// Give info about CSR
    Info {
        /// CSR
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Print { name, value } => match to_csr(&name, value) {
            Ok(csr) => csr.print(),
            Err(e) => eprint!("{}", e),
        },
        Commands::Info { name } => match to_csr(&name, 0) {
            Ok(csr) => print!("{}", csr.info()),
            Err(e) => eprint!("{}", e),
        },
    }
}
