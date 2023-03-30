use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

mod csr;

mod print;
use print::{PrintError, TColor};

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
            let print_res = print::print_csr(&name, value);
            match print_res {
                Ok(_) => (),
                Err(PrintError::UnkownCsr(csr)) => eprintln!("{}ERROR: \"{}\" is not a legal CSR name{}", TColor::RS, csr, TColor::N),
            }
        }
    }
}