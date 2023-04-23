use std::mem::size_of;

use thiserror::Error;

pub mod base;
pub mod h_ext;

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
        "0x100" | "sstatus" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x104" | "sie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x105" | "stvec" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x106" | "scounteren" => Err(CsrError::UnsupportedCsr(name.into())),
        // Supervisor config
        "0x10a" | "senvcfg" => Err(CsrError::UnsupportedCsr(name.into())),
        // Supervisor trap handling
        "0x140" | "sscratch" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x141" | "sepc" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x142" | "scause" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x143" | "stval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x144" | "sip" => Err(CsrError::UnsupportedCsr(name.into())),
        // Supervisor Protection and Translation
        "0x180" | "satp" => Ok(Box::new(self::base::Satp::new(value))),
        // Debug & Trace Registers
        "0x5a8" | "scontext" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Trap Setup
        "0x600" | "hstatus" => Ok(Box::new(self::h_ext::Hstatus::new(value))),
        "0x602" | "hedeleg" => Ok(Box::new(self::h_ext::Hedeleg::new(value))),
        "0x603" | "hideleg" => Ok(Box::new(self::h_ext::Hideleg::new(value))),
        "0x604" | "hie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x606" | "hcounteren" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x607" | "hgeie" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Trap Handling
        "0x643" | "htval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x644" | "hip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x645" | "hvip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x64a" | "htinst" => Err(CsrError::UnsupportedCsr(name.into())),
        "0xe12" | "hgeip" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor configuration
        "0x60a" | "henvcfg" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Protection and Translation
        "0x680" | "hgatp" => Err(CsrError::UnsupportedCsr(name.into())),
        // Debug/Trace Registers
        "0x6a8" | "hcontext" => Err(CsrError::UnsupportedCsr(name.into())),
        // Hypervisor Counter/Timer Virtualization Registers
        "0x605" | "htimedelta" => Err(CsrError::UnsupportedCsr(name.into())),
        // Virtual Supervisor Registers
        "0x200" | "vsstatus" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x204" | "vsie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x205" | "vstvec" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x240" | "vsscratch" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x241" | "vsepc" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x242" | "vscause" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x243" | "vstval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x244" | "vsip" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x280" | "vsatp" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Information Registers
        "0xf11" | "mvendorid" => Ok(Box::new(self::base::Mvendorid::new(value))),
        "0xf12" | "marchid" => Ok(Box::new(self::base::Marchid::new(value))),
        "0xf13" | "mimpid" => Ok(Box::new(self::base::Mimpid::new(value))),
        "0xf14" | "mhartid" => Ok(Box::new(self::base::Mhartid::new(value))),
        "0xf15" | "mconfigptr" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Trap Setup
        "0x300" | "mstatus" => Ok(Box::new(self::base::Mstatus::new(value))),
        "0x301" | "misa" => Ok(Box::new(self::base::Misa::new(value))),
        "0x302" | "medeleg" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x303" | "mideleg" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x304" | "mie" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x305" | "mtvec" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x306" | "mcounteren" => Err(CsrError::UnsupportedCsr(name.into())),
        // Machine Trap Handling
        "0x340" | "mscratch" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x341" | "mepc" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x342" | "mcause" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x343" | "mtval" => Err(CsrError::UnsupportedCsr(name.into())),
        "0x344" | "mip" => Err(CsrError::UnsupportedCsr(name.into())),
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

pub trait Csr {
    /// Create a new CSR instance from a value
    fn new(value: u64) -> Self
    where
        Self: Sized;

    /// Print CSR's value
    fn print(&self);
}

#[derive(Error, Debug)]
pub enum CsrError {
    #[error("\x1b[31m\x1b[1mERROR: \"{0}\" is not a legal CSR name\x1b[0m")]
    UnkownCsr(String),
    #[error("\x1b[33m\x1b[1mWARNING: \"{0}\" is not (yet) supported\x1b[0m")]
    UnsupportedCsr(String),
}

pub fn get_bit<I>(value: I, index: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= index);

    let value = value << (lastbit - index);
    let value = value >> (lastbit);
    value
}

pub fn get_bits<I>(value: I, start: usize, end: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= end);
    assert!(end >= start);

    let value = value << (lastbit - end);
    let value = value >> (lastbit - end + start);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_bit_u8_1() {
        assert_eq!(1, get_bit(0b0000_0010_u8, 1));
    }

    #[test]
    fn get_bit_u8_0() {
        assert_eq!(0, get_bit(0b1111_1011_u8, 2));
    }

    #[test]
    fn get_bit_u16_1() {
        assert_eq!(1, get_bit(0x4000_u16, 14));
    }

    #[test]
    fn get_bit_u16_0() {
        assert_eq!(0, get_bit(0xfdff_u16, 9));
    }

    #[test]
    fn get_bits_u16_l() {
        assert_eq!(3, get_bits(0b000_1100_u8, 2, 3))
    }

    #[test]
    fn get_bits_u16_m() {
        assert_eq!(0xf, get_bits(0b011_1100_u8, 2, 5))
    }
}