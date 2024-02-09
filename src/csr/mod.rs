use thiserror::Error;

use crate::encoding;
use crate::format::Csr;

// Export CSRs
mod m_level;
pub use m_level::*;
mod h_level;
pub use h_level::*;
mod s_level;
pub use s_level::*;
mod pmp;
pub use pmp::*;

/// Represents a CSR address
type Addr = u16;

/// Errors that may arise when creating/handling a CSR
#[derive(Error, Debug)]
pub enum CsrError {
    #[error("\"{0}\" is not a name of a supported CSR")]
    UnknownName(String),
    #[error("0x{0:03x} is not an address of a supported CSR")]
    UnkownAddr(Addr),
}

/// Convert a name/address string to a valid [Addr]
fn addr(csr_str: &str) -> Result<Addr, CsrError> {
    if let Some(addr_str) = csr_str.strip_prefix("0x") {
        let a = Addr::from_str_radix(addr_str, 16);
        Ok(a.expect("A 0x prefixed string should be a valid hexadecimal number"))
    } else if let Some(addr_str) = csr_str.strip_prefix("0b") {
        let a = Addr::from_str_radix(addr_str, 2);
        Ok(a.expect("A 0b prefixed string should be a valid binary number"))
    } else if let Ok(a) = Addr::from_str_radix(csr_str, 10) {
        Ok(a)
    } else if let Some(a) = encoding::csr_address_map(csr_str) {
        Ok(a)
    } else {
        Err(CsrError::UnknownName(csr_str.to_string()))
    }
}

/// Build a CSR [Format] from an address and value
pub fn format(csr_str: &str, value: u64) -> Result<Box<dyn Csr>, CsrError> {
    let address = match addr(csr_str) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    match address {
        // Unprivileged counters & timers
        encoding::CSR_CYCLE => todo!("Add cycle format"),
        encoding::CSR_TIME => todo!("Add time format"),
        encoding::CSR_INSTRET => todo!("Add instret format"),
        encoding::CSR_HPMCOUNTER3..=encoding::CSR_HPMCOUNTER31 => {
            todo!("Add hpmcounter format")
        }
        // Supervisor trap setup
        encoding::CSR_SSTATUS => Ok(Box::new(Sstatus::new(value))),
        encoding::CSR_SIE => Ok(Box::new(Sie::new(value))),
        encoding::CSR_STVEC => Ok(Box::new(Stvec::new(value))),
        encoding::CSR_SCOUNTEREN => Ok(Box::new(Scounteren::new(value))),
        // Supervisor config
        encoding::CSR_SENVCFG => Ok(Box::new(Senvcfg::new(value))),
        // Supervisor trap handling
        encoding::CSR_SSCRATCH => Ok(Box::new(Sscratch::new(value))),
        encoding::CSR_SEPC => Ok(Box::new(Sepc::new(value))),
        encoding::CSR_SCAUSE => Ok(Box::new(Scause::new(value))),
        encoding::CSR_STVAL => Ok(Box::new(Mtval::new(value))),
        encoding::CSR_SIP => Ok(Box::new(Sip::new(value))),
        // Supervisor Protection and Translation
        encoding::CSR_SATP => Ok(Box::new(Satp::new(value))),
        // Hypervisor Trap Setup
        encoding::CSR_HSTATUS => Ok(Box::new(Hstatus::new(value))),
        encoding::CSR_HEDELEG => Ok(Box::new(Hedeleg::new(value))),
        encoding::CSR_HIDELEG => Ok(Box::new(Hideleg::new(value))),
        encoding::CSR_HIE => todo!("Add hie format"),
        encoding::CSR_HCOUNTEREN => Ok(Box::new(Hcounteren::new(value))),
        encoding::CSR_HGEIE => todo!("Add hgeie format"),
        // Hypervisor Trap Handling
        encoding::CSR_HTVAL => todo!("Add htval format"),
        encoding::CSR_HIP => todo!("Add hip format"),
        encoding::CSR_HVIP => todo!("Add hvip format"),
        encoding::CSR_HTINST => todo!("Add htinst format"),
        encoding::CSR_HGEIP => todo!("Add hgeip format"),
        // Hypervisor configuration
        encoding::CSR_HENVCFG => Ok(Box::new(Henvcfg::new(value))),
        // Hypervisor Protection and Translation
        encoding::CSR_HGATP => Ok(Box::new(Hgatp::new(value))),
        // Hypervisor Counter/Timer Virtualization Registers
        encoding::CSR_HTIMEDELTA => todo!("Add htimedelta format"),
        // Virtual Supervisor Registers
        encoding::CSR_VSSTATUS => Ok(Box::new(Sstatus::new(value))),
        encoding::CSR_VSIE => todo!("Add vsie format"),
        encoding::CSR_VSTVEC => Ok(Box::new(Stvec::new(value))),
        encoding::CSR_VSSCRATCH => Ok(Box::new(Sscratch::new(value))),
        encoding::CSR_VSEPC => Ok(Box::new(Sepc::new(value))),
        encoding::CSR_VSCAUSE => Ok(Box::new(Scause::new(value))),
        encoding::CSR_VSTVAL => todo!("Add vstval format"),
        encoding::CSR_VSIP => todo!("Add vsip format"),
        encoding::CSR_VSATP => Ok(Box::new(Satp::new(value))),
        // Machine Information Registers
        encoding::CSR_MVENDORID => Ok(Box::new(Mvendorid::new(value))),
        encoding::CSR_MARCHID => Ok(Box::new(Marchid::new(value))),
        encoding::CSR_MIMPID => Ok(Box::new(Mimpid::new(value))),
        encoding::CSR_MHARTID => Ok(Box::new(Mhartid::new(value))),
        // Machine Trap Setup
        encoding::CSR_MSTATUS => Ok(Box::new(Mstatus::new(value))),
        encoding::CSR_MISA => Ok(Box::new(Misa::new(value))),
        encoding::CSR_MEDELEG => Ok(Box::new(Medeleg::new(value))),
        encoding::CSR_MIDELEG => Ok(Box::new(Mideleg::new(value))),
        encoding::CSR_MIE => Ok(Box::new(Mie::new(value))),
        encoding::CSR_MTVEC => Ok(Box::new(Mtvec::new(value))),
        encoding::CSR_MCOUNTEREN => todo!("Add mcounteren format"),
        // Machine Trap Handling
        encoding::CSR_MSCRATCH => todo!("Add mscratch format"),
        encoding::CSR_MEPC => todo!("Add mepc format"),
        encoding::CSR_MCAUSE => todo!("Add mcause format"),
        encoding::CSR_MTVAL => Ok(Box::new(Mtval::new(value))),
        encoding::CSR_MIP => Ok(Box::new(Mip::new(value))),
        encoding::CSR_MTINST => todo!("Add mtinst format"),
        encoding::CSR_MTVAL2 => Ok(Box::new(Mtval2::new(value))),
        // Machine Configuration
        encoding::CSR_MENVCFG => todo!("Add machine environment config format"),
        encoding::CSR_MSECCFG => todo!("Add machine security config format"),
        // Physical memory protection
        encoding::CSR_PMPADDR0..=encoding::CSR_PMPADDR63 => Ok(Box::new(PmpAddr::new(value))),
        encoding::CSR_PMPCFG0..=encoding::CSR_PMPCFG15 => Ok(Box::new(PmpCfg::new(value))),
        // Unkown CSR
        _ => Err(CsrError::UnkownAddr(address)),
    }
}
