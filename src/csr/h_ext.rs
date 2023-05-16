//! CSR definitions for the hypervisor extension
use crate::bitfield::BitField;

use super::base::dec_arch;
use super::Csr;

/**************************************************************/
/* Hypervisor Status Register                                 */
pub struct Hstatus {
    pub vsbe: BitField<5, 5>,
    pub gva: BitField<6, 6>,
    pub spv: BitField<7, 7>,
    pub spvp: BitField<8, 8>,
    pub hu: BitField<9, 9>,
    pub vgein: BitField<12, 17>,
    pub vtvm: BitField<20, 20>,
    pub vtw: BitField<21, 21>,
    pub vtsr: BitField<22, 22>,
    pub vsxl: BitField<32, 33>,
}

impl Csr for Hstatus {
    fn new(value: u64) -> Self {
        Hstatus {
            vsbe: value.into(),
            gva: value.into(),
            spv: value.into(),
            spvp: value.into(),
            hu: value.into(),
            vgein: value.into(),
            vtvm: value.into(),
            vtw: value.into(),
            vtsr: value.into(),
            vsxl: value.into(),
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
        println!("VSXL: {}", dec_arch(self.vsxl.value()));
    }
}

/**************************************************************/
/* Hypervisor Trap Delegation Registers                       */
pub struct Hedeleg(BitField<0, 63>);

impl Csr for Hedeleg {
    fn new(value: u64) -> Hedeleg {
        Hedeleg(value.into())
    }

    fn print(&self) {
        println!("hedeleg: {}", &self.0);
    }
}

pub struct Hideleg(BitField<0, 63>);

impl Csr for Hideleg {
    fn new(value: u64) -> Hideleg {
        Hideleg(value.into())
    }

    fn print(&self) {
        println!("hideleg: {}", &self.0);
    }
}
