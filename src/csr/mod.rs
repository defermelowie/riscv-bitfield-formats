use std::fmt::Display;

use thiserror::Error;

// Export CSRs
mod m_level;
pub use m_level::*;
mod h_level;
pub use h_level::*;
mod s_level;
pub use s_level::*;

/// Get a csr from a name/address and value
pub fn to_csr(name: &str, value: u64) -> Result<Box<dyn Csr>, CsrError> {
    match name {
        // Unprivileged floating-point
        "0x001" | "fflags" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x002" | "frm" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x003" | "fcsr" => Err(CsrError::UnsupportedCsr(name.into())),
        // Unprivileged counters & timers
        "0xc00" | "cycle" => Err(CsrError::UnsupportedCsr(name.into())),
        "0xc01" | "time" => Err(CsrError::UnsupportedCsr(name.into())),
        "0xc02" | "instret" => Err(CsrError::UnsupportedCsr(name.into())),
        "hmpcounter" => Err(CsrError::UnsupportedCsr(name.into())),
        // Supervisor trap setup
        "0x100" | "sstatus" => Ok(Box::new(Sstatus::new(value))),
        "0x104" | "sie" => Ok(Box::new(Sie::new(value))),
        "0x105" | "stvec" => Ok(Box::new(Stvec::new(value))),
        "0x106" | "scounteren" => Ok(Box::new(Scounteren::new(value))),
        // Supervisor config
        "0x10a" | "senvcfg" => Ok(Box::new(Senvcfg::new(value))),
        // Supervisor trap handling
        "0x140" | "sscratch" => Ok(Box::new(Sscratch::new(value))),
        "0x141" | "sepc" => Ok(Box::new(Sepc::new(value))),
        "0x142" | "scause" => Ok(Box::new(Scause::new(value))),
        "0x143" | "stval" => Ok(Box::new(Stval::new(value))),
        "0x144" | "sip" => Ok(Box::new(Sip::new(value))),
        // Supervisor Protection and Translation
        "0x180" | "satp" => Ok(Box::new(Satp::new(value))),
        // Debug & Trace Registers
        "0x5a8" | "scontext" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Trap Setup
        "0x600" | "hstatus" => Ok(Box::new(Hstatus::new(value))),
        "0x602" | "hedeleg" => Ok(Box::new(Hedeleg::new(value))),
        "0x603" | "hideleg" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x604" | "hie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x606" | "hcounteren" => Ok(Box::new(Hcounteren::new(value))),
        "0x607" | "hgeie" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Trap Handling
        "0x643" | "htval" => Ok(Box::new(Htval::new(value))),
        "0x644" | "hip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x645" | "hvip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x64a" | "htinst" => Err(CsrError::UnsupportedCsr(name.into())),
        "0xe12" | "hgeip" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor configuration
        "0x60a" | "henvcfg" => Ok(Box::new(Henvcfg::new(value))),
        // Hypervisor Protection and Translation
        "0x680" | "hgatp" => Ok(Box::new(Hgatp::new(value))),
        // Debug/Trace Registers
        "0x6a8" | "hcontext" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Counter/Timer Virtualization Registers
        "0x605" | "htimedelta" => Err(CsrError::UnsupportedCsr(name.into())),
        // Virtual Supervisor Registers
        "0x200" | "vsstatus" => Ok(Box::new(Sstatus::new(value))),
        "0x204" | "vsie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x205" | "vstvec" => Ok(Box::new(Stvec::new(value))),
        "0x240" | "vsscratch" => Ok(Box::new(Sscratch::new(value))),
        "0x241" | "vsepc" => Ok(Box::new(Sepc::new(value))),
        "0x242" | "vscause" => Ok(Box::new(Scause::new(value))),
        "0x243" | "vstval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x244" | "vsip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x280" | "vsatp" => Ok(Box::new(Satp::new(value))),
        // Machine Information Registers
        "0xf11" | "mvendorid" => Ok(Box::new(Mvendorid::new(value))),
        "0xf12" | "marchid" => Ok(Box::new(Marchid::new(value))),
        "0xf13" | "mimpid" => Ok(Box::new(Mimpid::new(value))),
        "0xf14" | "mhartid" => Ok(Box::new(Mhartid::new(value))),
        "0xf15" | "mconfigptr" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Trap Setup
        "0x300" | "mstatus" => Ok(Box::new(Mstatus::new(value))),
        "0x301" | "misa" => Ok(Box::new(Misa::new(value))),
        "0x302" | "medeleg" => Ok(Box::new(Medeleg::new(value))),
        "0x303" | "mideleg" => Ok(Box::new(Mideleg::new(value))),
        "0x304" | "mie" => Ok(Box::new(Mie::new(value))),
        "0x305" | "mtvec" => Ok(Box::new(Mtvec::new(value))),
        "0x306" | "mcounteren" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Trap Handling
        "0x340" | "mscratch" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x341" | "mepc" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x342" | "mcause" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x343" | "mtval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x344" | "mip" => Ok(Box::new(Mip::new(value))),
        "0x34A" | "mtinst" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x34B" | "mtval2" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Configuration
        "0x30A" | "menvcfg" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x747" | "mseccfg" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Non-Maskable Interrupt Handling
        "0x740" | "mnscratch" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x741" | "mnepc" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x742" | "mncause" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x744" | "mnstatus" => Err(CsrError::UnsupportedCsr(name.into())),
        // Unkown CSR
        _ => Err(CsrError::UnkownCsr(name.into())),
    }
}

pub trait Csr
where
    Self: Display,
{
    /// Create a new CSR instance from a value
    fn new(value: u64) -> Self
    where
        Self: Sized;

    /// Get CSR's name
    fn name(&self) -> String;
}

#[derive(Error, Debug)]
pub enum CsrError {
    #[error("\x1b[31m\x1b[1mERROR: \"{0}\" is not a known legal CSR name\x1b[0m")]
    UnkownCsr(String),
    #[error("\x1b[33m\x1b[1mWARNING: \"{0}\" is not (yet) supported\x1b[0m")]
    UnsupportedCsr(String),
}
