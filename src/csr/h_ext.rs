//! CSR definitions for the hypervisor extension
use super::base::dec_arch;
use super::{get_bit, get_bits, Csr};
use bits::{Bits, B1, B2, B6, B64};

/**************************************************************/
/* Hypervisor Status Register                                 */

pub struct Hstatus {
    pub vsbe: B1,
    pub gva: B1,
    pub spv: B1,
    pub spvp: B1,
    pub hu: B1,
    pub vgein: B6,
    pub vtvm: B1,
    pub vtw: B1,
    pub vtsr: B1,
    pub vsxl: B2,
}

impl Csr for Hstatus {
    fn new(value: u64) -> Self {
        Hstatus {
            vsbe: B1(get_bit(value, 5)),
            gva: B1(get_bit(value, 6)),
            spv: B1(get_bit(value, 7)),
            spvp: B1(get_bit(value, 8)),
            hu: B1(get_bit(value, 9)),
            vgein: B6(get_bits(value, 12, 17)),
            vtvm: B1(get_bit(value, 20)),
            vtw: B1(get_bit(value, 21)),
            vtsr: B1(get_bit(value, 22)),
            vsxl: B2(get_bits(value, 32, 33)),
        }
    }

    fn print(&self) {
        println!("");
        println!("hstatus");
        println!("-------");
        println!("VSBE: {}", &self.vsbe);
        println!("GVA: {}", &self.gva);
        println!("SPV: {}", &self.spv);
        println!("SPVP: {}", &self.spvp);
        println!("HU: {}", &self.hu);
        println!("VGEIN: {}", &self.vgein);
        println!("VTVM: {}", &self.vtvm);
        println!("VTW: {}", &self.vtw);
        println!("VTSR: {}", &self.vtsr);
        println!("VSXL: {}", dec_arch(self.vsxl.to_u64()));
    }
}

/**************************************************************/
/* Hypervisor Trap Delegation Registers                       */

pub struct Hedeleg(B64);

impl Csr for Hedeleg {
    fn new(value: u64) -> Hedeleg {
        Hedeleg(B64(value))
    }

    fn print(&self) {
        println!("hedeleg: {}", &self.0);
    }
}

pub struct Hideleg(B64);

impl Csr for Hideleg {
    fn new(value: u64) -> Hideleg {
        Hideleg(B64(value))
    }

    fn print(&self) {
        println!("hideleg: {}", &self.0);
    }
}
