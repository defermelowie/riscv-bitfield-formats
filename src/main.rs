use std::process::exit;

use clap::Parser;
use clap_num::maybe_hex;

mod bitfield;
mod encoding;
mod format;

mod csr;
mod inst;
mod vmem;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// CSR
    name: String,

    /// Value
    #[clap(value_parser=maybe_hex::<u64>)]
    value: u64,
}

fn main() -> ! {
    let cli = Cli::parse();

    // Try to format as instruction
    if let "ins" | "inst" | "instr" = cli.name.as_str() {
        todo!("Format instructions")
        // let inst = inst::format(cli.value);
        // print!("{}", inst);
        // exit(0)
    }

    // Try to format as CSR
    let csr = csr::format(&cli.name, cli.value);
    if let Ok(csr) = csr {
        print!("{}", csr);
        exit(0)
    }

    // Try to format virtual memory related
    let vmem = vmem::format(&cli.name, cli.value);
    if let Ok(vmem) = vmem {
        print!("{}", vmem);
        exit(0)
    }

    // Output error if formatting failed
    eprint!("\x1b[31m\x1b[1m");
    eprintln!("Formatting failed:");
    eprintln!("\t- {}", csr.err().unwrap());
    eprintln!("\t- {}", vmem.err().unwrap());
    eprint!("\x1b[0m");
    exit(-1)
}
