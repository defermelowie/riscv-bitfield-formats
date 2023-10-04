use std::fmt::Display;

use thiserror::Error;

// Export CSRs
mod m_level;
pub use m_level::*;
mod h_level;
pub use h_level::*;
mod s_level;
pub use s_level::*;
mod vmem;
pub use vmem::*;

/// Get a csr from a name/address and value
pub fn to_csr(name: &str, value: u64) -> Result<Box<dyn Csr>, CsrError> {
    match &*name.to_lowercase() {
        // Unprivileged floating-point
        "0x001" | "fflags" => Err(CsrError::Unsupported(name.into())),
        "0x002" | "frm" => Err(CsrError::Unsupported(name.into())),
        "0x003" | "fcsr" => Err(CsrError::Unsupported(name.into())),
        // Unprivileged counters & timers
        "0xc00" | "cycle" => Err(CsrError::Unsupported(name.into())),
        "0xc01" | "time" => Err(CsrError::Unsupported(name.into())),
        "0xc02" | "instret" => Err(CsrError::Unsupported(name.into())),
        "hmpcounter" => Err(CsrError::Unsupported(name.into())),
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
        "0x143" | "stval" => Ok(Box::new(Mtval::new(value))),
        "0x144" | "sip" => Ok(Box::new(Sip::new(value))),
        // Supervisor Protection and Translation
        "0x180" | "satp" => Ok(Box::new(Satp::new(value))),
        // Debug & Trace Registers
        "0x5a8" | "scontext" => Err(CsrError::Unsupported(name.into())),
        // Hypervisor Trap Setup
        "0x600" | "hstatus" => Ok(Box::new(Hstatus::new(value))),
        "0x602" | "hedeleg" => Ok(Box::new(Hedeleg::new(value))),
        "0x603" | "hideleg" => Err(CsrError::Unsupported(name.into())),
        "0x604" | "hie" => Err(CsrError::Unsupported(name.into())),
        "0x606" | "hcounteren" => Ok(Box::new(Hcounteren::new(value))),
        "0x607" | "hgeie" => Err(CsrError::Unsupported(name.into())),
        // Hypervisor Trap Handling
        "0x643" | "htval" => Ok(Box::new(Htval::new(value))),
        "0x644" | "hip" => Err(CsrError::Unsupported(name.into())),
        "0x645" | "hvip" => Err(CsrError::Unsupported(name.into())),
        "0x64a" | "htinst" => Err(CsrError::Unsupported(name.into())),
        "0xe12" | "hgeip" => Err(CsrError::Unsupported(name.into())),
        // Hypervisor configuration
        "0x60a" | "henvcfg" => Ok(Box::new(Henvcfg::new(value))),
        // Hypervisor Protection and Translation
        "0x680" | "hgatp" => Ok(Box::new(Hgatp::new(value))),
        // Debug/Trace Registers
        "0x6a8" | "hcontext" => Err(CsrError::Unsupported(name.into())),
        // Hypervisor Counter/Timer Virtualization Registers
        "0x605" | "htimedelta" => Err(CsrError::Unsupported(name.into())),
        // Virtual Supervisor Registers
        "0x200" | "vsstatus" => Ok(Box::new(Sstatus::new(value))),
        "0x204" | "vsie" => Err(CsrError::Unsupported(name.into())),
        "0x205" | "vstvec" => Ok(Box::new(Stvec::new(value))),
        "0x240" | "vsscratch" => Ok(Box::new(Sscratch::new(value))),
        "0x241" | "vsepc" => Ok(Box::new(Sepc::new(value))),
        "0x242" | "vscause" => Ok(Box::new(Scause::new(value))),
        "0x243" | "vstval" => Err(CsrError::Unsupported(name.into())),
        "0x244" | "vsip" => Err(CsrError::Unsupported(name.into())),
        "0x280" | "vsatp" => Ok(Box::new(Satp::new(value))),
        // Machine Information Registers
        "0xf11" | "mvendorid" => Ok(Box::new(Mvendorid::new(value))),
        "0xf12" | "marchid" => Ok(Box::new(Marchid::new(value))),
        "0xf13" | "mimpid" => Ok(Box::new(Mimpid::new(value))),
        "0xf14" | "mhartid" => Ok(Box::new(Mhartid::new(value))),
        "0xf15" | "mconfigptr" => Err(CsrError::Unsupported(name.into())),
        // Machine Trap Setup
        "0x300" | "mstatus" => Ok(Box::new(Mstatus::new(value))),
        "0x301" | "misa" => Ok(Box::new(Misa::new(value))),
        "0x302" | "medeleg" => Ok(Box::new(Medeleg::new(value))),
        "0x303" | "mideleg" => Ok(Box::new(Mideleg::new(value))),
        "0x304" | "mie" => Ok(Box::new(Mie::new(value))),
        "0x305" | "mtvec" => Ok(Box::new(Mtvec::new(value))),
        "0x306" | "mcounteren" => Err(CsrError::Unsupported(name.into())),
        // Machine Trap Handling
        "0x340" | "mscratch" => Err(CsrError::Unsupported(name.into())),
        "0x341" | "mepc" => Err(CsrError::Unsupported(name.into())),
        "0x342" | "mcause" => Err(CsrError::Unsupported(name.into())),
        "0x343" | "mtval" => Ok(Box::new(Mtval::new(value))),
        "0x344" | "mip" => Ok(Box::new(Mip::new(value))),
        "0x34a" | "mtinst" => Err(CsrError::Unsupported(name.into())),
        "0x34b" | "mtval2" => Ok(Box::new(Mtval2::new(value))),
        // Machine Configuration
        "0x30a" | "menvcfg" => Err(CsrError::Unsupported(name.into())),
        "0x747" | "mseccfg" => Err(CsrError::Unsupported(name.into())),
        // Machine Non-Maskable Interrupt Handling
        "0x740" | "mnscratch" => Err(CsrError::Unsupported(name.into())),
        "0x741" | "mnepc" => Err(CsrError::Unsupported(name.into())),
        "0x742" | "mncause" => Err(CsrError::Unsupported(name.into())),
        "0x744" | "mnstatus" => Err(CsrError::Unsupported(name.into())),
        // NOTE: Not CSRs but still usefull to be able to format
        "pte_sv32" | "sv32_pte" => Ok(Box::new(Pte32::new(value))),
        "pte_sv39" | "sv39_pte" => Ok(Box::new(Pte39::new(value))),
        "pte_sv48" | "sv48_pte" => Ok(Box::new(Pte48::new(value))),
        "pte_sv57" | "sv57_pte" => Ok(Box::new(Pte57::new(value))),
        "vaddr_sv32" | "sv32_vaddr" => Ok(Box::new(VAddr32::new(value))),
        "vaddr_sv39" | "sv39_vaddr" => Ok(Box::new(VAddr39::new(value))),
        "vaddr_sv48" | "sv48_vaddr" => Ok(Box::new(VAddr48::new(value))),
        "vaddr_sv57" | "sv57_vaddr" => Ok(Box::new(VAddr57::new(value))),
        "paddr_sv32" | "sv32_paddr" => Ok(Box::new(PAddr32::new(value))),
        "paddr_sv39" | "sv39_paddr" => Ok(Box::new(PAddr39::new(value))),
        "paddr_sv48" | "sv48_paddr" => Ok(Box::new(PAddr48::new(value))),
        "paddr_sv57" | "sv57_paddr" => Ok(Box::new(PAddr57::new(value))),
        // Unkown CSR
        _ => Err(CsrError::Unkown(name.into())),
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
    fn name() -> String
    where
        Self: Sized;
}

#[derive(Error, Debug)]
pub enum CsrError {
    #[error("\x1b[31m\x1b[1mERROR: \"{0}\" is not known\x1b[0m")]
    Unkown(String),
    #[error("\x1b[33m\x1b[1mWARNING: \"{0}\" is not (yet) supported\x1b[0m")]
    Unsupported(String),
}
