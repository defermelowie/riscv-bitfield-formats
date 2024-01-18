//! Definitions for the hypervisor extended supervisor mode CSRs
use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Arch, Atp, Bin, Hex, Reserved};

/// Hypervisor Status Register
#[derive(Csr)]
pub struct Hstatus {
    vsbe: BitField<Bin, 5, 5>,
    gva: BitField<Bin, 6, 6>,
    spv: BitField<Bin, 7, 7>,
    spvp: BitField<Bin, 8, 8>,
    hu: BitField<Bin, 9, 9>,
    vgein: BitField<Bin, 12, 17>,
    vtvm: BitField<Bin, 20, 20>,
    vtw: BitField<Bin, 21, 21>,
    vtsr: BitField<Bin, 22, 22>,
    vsxl: BitField<Arch, 32, 33>,
}

/// Hypervisor Exception Delegation Register
#[derive(Csr)]
pub struct Hedeleg {
    misaligned_fetch: BitField<Bin, 0x0, 0x0>,
    fetch_access: BitField<Bin, 0x1, 0x1>,
    illegal_instruction: BitField<Bin, 0x2, 0x2>,
    breakpoint: BitField<Bin, 0x3, 0x3>,
    misaligned_load: BitField<Bin, 0x4, 0x4>,
    load_access: BitField<Bin, 0x5, 0x5>,
    misaligned_store: BitField<Bin, 0x6, 0x6>,
    store_access: BitField<Bin, 0x7, 0x7>,
    user_ecall: BitField<Bin, 0x8, 0x8>,
    supervisor_ecall: BitField<Bin, 0x9, 0x9>,
    virtual_supervisor_ecall: BitField<Bin, 0xa, 0xa>,
    machine_ecall: BitField<Bin, 0xb, 0xb>,
    fetch_page_fault: BitField<Bin, 0xc, 0xc>,
    load_page_fault: BitField<Bin, 0xd, 0xd>,
    store_page_fault: BitField<Bin, 0xf, 0xf>,
    fetch_guest_page_fault: BitField<Bin, 0x14, 0x14>,
    load_guest_page_fault: BitField<Bin, 0x15, 0x15>,
    virtual_instruction: BitField<Bin, 0x16, 0x16>,
    store_guest_page_fault: BitField<Bin, 0x17, 0x17>,
}

/// Hypervisor Interrupt Delegation Register
#[derive(Csr)]
pub struct Hideleg {
    supervisor_sw_interrupt: BitField<Reserved<0, Bin>, 1, 1>,
    virtual_supervisor_sw_interrupt: BitField<Bin, 2, 2>,
    machine_sw_interrupt: BitField<Bin, 3, 3>,
    supervisor_timer_interrupt: BitField<Reserved<0, Bin>, 5, 5>,
    virtual_supervisor_timer_interrupt: BitField<Bin, 6, 6>,
    machine_timer_interrupt: BitField<Bin, 7, 7>,
    supervisor_external_interrupt: BitField<Reserved<0, Bin>, 9, 9>,
    virtual_supervisor_external_interrupt: BitField<Bin, 10, 10>,
    machine_external_interrupt: BitField<Bin, 11, 11>,
    supervisor_guest_external_interrupt: BitField<Reserved<0, Bin>, 12, 12>,
    custom: BitField<Hex, 16, 63>,
}

/// Hypervisor Counter Enable Register
#[derive(Csr)]
pub struct Hcounteren {
    hpm31: BitField<Bin, 31, 31>,
    hpm30: BitField<Bin, 30, 30>,
    hpm29: BitField<Bin, 29, 29>,
    hpm28: BitField<Bin, 28, 28>,
    hpm27: BitField<Bin, 27, 27>,
    hpm26: BitField<Bin, 26, 26>,
    hpm25: BitField<Bin, 25, 25>,
    hpm24: BitField<Bin, 24, 24>,
    hpm23: BitField<Bin, 23, 23>,
    hpm22: BitField<Bin, 22, 22>,
    hpm21: BitField<Bin, 21, 21>,
    hpm20: BitField<Bin, 20, 20>,
    hpm19: BitField<Bin, 19, 19>,
    hpm18: BitField<Bin, 18, 18>,
    hpm17: BitField<Bin, 17, 17>,
    hpm16: BitField<Bin, 16, 16>,
    hpm15: BitField<Bin, 15, 15>,
    hpm14: BitField<Bin, 14, 14>,
    hpm13: BitField<Bin, 13, 13>,
    hpm12: BitField<Bin, 12, 12>,
    hpm11: BitField<Bin, 11, 11>,
    hpm10: BitField<Bin, 10, 10>,
    hpm9: BitField<Bin, 9, 9>,
    hpm8: BitField<Bin, 8, 8>,
    hpm7: BitField<Bin, 7, 7>,
    hpm6: BitField<Bin, 6, 6>,
    hpm5: BitField<Bin, 5, 5>,
    hpm4: BitField<Bin, 4, 4>,
    hpm3: BitField<Bin, 3, 3>,
    ir: BitField<Bin, 2, 2>,
    tm: BitField<Bin, 1, 1>,
    cy: BitField<Bin, 0, 0>,
}

/// Hypervisor Trap Value Register
#[derive(Csr)]
pub struct Htval {
    val: BitField<Hex, 0, 63>,
}

///Hypervisor Trap Instruction Register
#[derive(Csr)]
pub struct Htinst {
    inst: BitField<Hex, 0, 63>,
}

/// Hypervisor Environment Configuration Register
#[derive(Csr)]
pub struct Henvcfg {
    stce: BitField<Bin, 63, 63>,
    pbmte: BitField<Bin, 62, 62>,
    cbze: BitField<Bin, 7, 7>,
    cbcfe: BitField<Bin, 6, 6>,
    cbie: BitField<Bin, 4, 5>,
    fiom: BitField<Bin, 0, 0>,
}

/// Hypervisor Guest Address Translation & Protection Register
#[derive(Csr)]
pub struct Hgatp {
    mode: BitField<Atp, 60, 63>,
    vmid: BitField<Hex, 44, 57>,
    ppn: BitField<Hex, 0, 43>,
}
