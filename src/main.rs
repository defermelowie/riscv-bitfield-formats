use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

mod csr;
mod print;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What do you want to do?
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints CSR's value in a human readable format
    Print {
        /// CSR
        name: String,

        /// Value
        #[clap(value_parser=maybe_hex::<u64>)]
        value: u64,
    },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Print{name, value} => {
            print::print_csr(&name, value);
        }
    }
}