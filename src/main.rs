use clap::Parser;
use clap_num::maybe_hex;

mod bitfield;

mod csr;
use csr::to_csr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// CSR
    name: String,

    /// Value
    #[clap(value_parser=maybe_hex::<u64>)]
    value: u64,
}

fn main() {
    let cli = Cli::parse();
    match to_csr(&cli.name, cli.value) {
        Ok(csr) => print!("{}", csr),
        Err(e) => eprint!("{}", e),
    }
}
