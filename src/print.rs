use crate::csr::{self, Csr};

pub fn print_csr(name: &str, value: u64) -> Result<(), PrintError> {
    let csr: Box<dyn Csr> = match name {
        // Unprivileged floating-point
        "fflags" => todo!(),
        "frm" => todo!(),
        "fcsr" => todo!(),
        // Unprivileged counters/timers
        "cycle" => todo!(),
        "time" => todo!(),
        "instret" => todo!(),
        "hmpcounter" => todo!(),
        // Supervisor trap setup
        "sstatus" => todo!(),
        "sie" => todo!(),
        "stvec" => todo!(),
        "scounteren" => todo!(),
        // Supervisor config
        "senvcfg" => todo!(),
        // Supervisor trap handling
        "sscratch" => todo!(),
        "sepc" => todo!(),
        "scause" => todo!(),
        "stval" => todo!(),
        "sip" => todo!(),

        // Machine config
        "misa" => Box::new(csr::base::Misa::new(value)),
        "mvendorid" => Box::new(csr::base::Mvendorid::new(value)),
        "marchid" => Box::new(csr::base::Marchid::new(value)),
        "mimpid" => Box::new(csr::base::Mimpid::new(value)),
        "mhartid" => Box::new(csr::base::Mhartid::new(value)),
        "mstatus" => Box::new(csr::base::Mstatus::new(value)),
        // hypervisor
        "hstatus" => Box::new(csr::h_ext::Hstatus::new(value)),
        "hedeleg" => Box::new(csr::h_ext::Hedeleg::new(value)),
        "hideleg" => Box::new(csr::h_ext::Hideleg::new(value)),
        // unkown
        _ => return Err(PrintError::UnkownCsr(name.into())),
    };
    csr.print();
    Ok(())
}

pub enum PrintError {
    UnkownCsr(String),
}

pub enum TColor {
    /// Red
    R,
    /// Blue
    B,
    /// Green
    G,
    /// Strong
    S,
    /// Normal
    N,
    /// Red-strong
    RS,
    /// Blue-strong
    BS,
    /// Green-strong
    GS,
}

impl std::fmt::Display for TColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_str = match &self {
            TColor::R => "\x1b[31m",
            TColor::B => "\x1b[36m",
            TColor::G => "\x1b[32m",
            TColor::S => "\x1b[1m",
            TColor::N => "\x1b[0m",
            TColor::RS => "\x1b[31m\x1b[1m",
            TColor::BS => "\x1b[36m\x1b[1m",
            TColor::GS => "\x1b[32m\x1b[1m",
        };
        write!(f, "{}", color_str)
    }
}
