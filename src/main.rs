use std::process::exit;

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

mod csr;
use csr::{to_csr, CsrError};

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
        Commands::Print { name, value } => {
            let csr = to_csr(&name, value);
            match csr {
                Ok(csr) => {
                    csr.print();
                    exit(0);
                }
                Err(CsrError::UnkownCsr(csr)) => {
                    eprintln!(
                        "\x1b[31m\x1b[1mERROR: \"{}\" is not a legal CSR name\x1b[0m",
                        csr
                    );
                    exit(-1);
                }
                Err(CsrError::UnsupportedCsr(csr)) => {
                    eprintln!(
                        "\x1b[33m\x1b[1mWARNING: \"{}\" is not (yet) supported\x1b[0m",
                        csr
                    );
                    exit(-1);
                }
            }
        }
    }
}
