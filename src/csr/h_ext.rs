//! CSR definitions for the hypervisor extension
use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Arch, Bin, Hex, Priv, Atp, Tvec};

/**************************************************************/
/* Hypervisor Status Register                                 */

#[derive(Csr)]
pub struct Hstatus {
    pub vsbe: BitField<Bin, 5, 5>,
    pub gva: BitField<Bin, 6, 6>,
    pub spv: BitField<Bin, 7, 7>,
    pub spvp: BitField<Bin, 8, 8>,
    pub hu: BitField<Bin, 9, 9>,
    pub vgein: BitField<Bin, 12, 17>,
    pub vtvm: BitField<Bin, 20, 20>,
    pub vtw: BitField<Bin, 21, 21>,
    pub vtsr: BitField<Bin, 22, 22>,
    pub vsxl: BitField<Arch, 32, 33>,
}

/**************************************************************/
/* Hypervisor Trap Delegation Registers                       */

#[derive(Csr)]
pub struct Hedeleg {
    pub misaligned_fetch: BitField<Bin, 0x0, 0x0>,
    pub fetch_access: BitField<Bin, 0x1, 0x1>,
    pub illegal_instruction: BitField<Bin, 0x2, 0x2>,
    pub breakpoint: BitField<Bin, 0x3, 0x3>,
    pub misaligned_load: BitField<Bin, 0x4, 0x4>,
    pub load_access: BitField<Bin, 0x5, 0x5>,
    pub misaligned_store: BitField<Bin, 0x6, 0x6>,
    pub store_access: BitField<Bin, 0x7, 0x7>,
    pub user_ecall: BitField<Bin, 0x8, 0x8>,
    pub supervisor_ecall: BitField<Bin, 0x9, 0x9>,
    pub virtual_supervisor_ecall: BitField<Bin, 0xa, 0xa>,
    pub machine_ecall: BitField<Bin, 0xb, 0xb>,
    pub fetch_page_fault: BitField<Bin, 0xc, 0xc>,
    pub load_page_fault: BitField<Bin, 0xd, 0xd>,
    pub store_page_fault: BitField<Bin, 0xf, 0xf>,
    pub fetch_guest_page_fault: BitField<Bin, 0x14, 0x14>,
    pub load_guest_page_fault: BitField<Bin, 0x15, 0x15>,
    pub virtual_instruction: BitField<Bin, 0x16, 0x16>,
    pub store_guest_page_fault: BitField<Bin, 0x17, 0x17>,
}

// #[derive(Csr)]
// pub struct Hideleg(BitField<0, 63>);
