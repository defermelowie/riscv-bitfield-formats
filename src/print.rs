use crate::csr::{self, Csr};

pub fn print_csr(name: &str, value: u64) {
    let csr: Box<dyn Csr> = match name {
        // base
        "misa"    => Box::new(csr::base::Misa::new(value)),
        // hypervisor extension
        "hstatus" => Box::new(csr::h_ext::Hstatus::new(value)),
        "hedeleg" => Box::new(csr::h_ext::Hedeleg::new(value)),
        "hideleg" => Box::new(csr::h_ext::Hideleg::new(value)),
        _ => panic!("{}\"{}\" is not a known CSR{}", TCLR.rs, name, TCLR.n),
    };
    csr.print();
}

struct TColor {
    /// Red
    r: &'static str,
    /// Blue
    b: &'static str,
    /// Green
    g: &'static str,
    /// Strong
    s: &'static str,
    /// Normal
    n: &'static str,
    /// Red-strong
    rs: &'static str,
    /// Blue-strong
    bs: &'static str,
    /// Green-strong
    gs: &'static str,
}

const TCLR: TColor = TColor {
    r: "\x1b[31m",
    b: "\x1b[36m",
    g: "\x1b[32m",
    s: "\x1b[1m",
    n: "\x1b[0m",
    rs: "\x1b[31m\x1b[1m",
    bs: "\x1b[36m\x1b[1m",
    gs: "\x1b[32m\x1b[1m",
};