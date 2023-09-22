//! Definitions for the supervisor mode CSRs
use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Arch, Atp, Bin, Bool, ExcCode, Hex, Priv, Tvec};

/// Supervisor Status Register
#[derive(Csr)]
pub struct Sstatus {
    sie: BitField<Bin, 1, 1>,
    spie: BitField<Bin, 5, 5>,
    ube: BitField<Bin, 6, 6>,
    spp: BitField<Priv, 8, 8>,
    vs: BitField<Bin, 9, 10>,
    fs: BitField<Bin, 13, 14>,
    xs: BitField<Bin, 15, 16>,
    sum: BitField<Bin, 18, 18>,
    mxr: BitField<Bin, 19, 19>,
    uxl: BitField<Arch, 32, 33>,
    sd: BitField<Bin, 63, 63>,
}

/// Supervisor Interrupt Enable Register
#[derive(Csr)]
pub struct Sie {
    supervisor_sw_interrupt: BitField<Bin, 1, 1>,
    supervisor_timer_interrupt: BitField<Bin, 5, 5>,
    supervisor_external_interrupt: BitField<Bin, 9, 9>,
}

/// Supervisor Interrupt Pending Register
#[derive(Csr)]
pub struct Sip {
    supervisor_sw_interrupt: BitField<Bin, 1, 1>,
    supervisor_timer_interrupt: BitField<Bin, 5, 5>,
    supervisor_external_interrupt: BitField<Bin, 9, 9>,
}

/// Supervisor Environment Configuration Register
#[derive(Csr)]
pub struct Senvcfg {
    fiom: BitField<Bin, 0, 0>,
    cbie: BitField<Bin, 4, 5>,
    cbcfe: BitField<Bin, 7, 7>,
}

/// Supervisor Trap Value Register
#[derive(Csr)]
pub struct Stval {
    stval: BitField<Hex, 0, 63>,
}

/// Supervisor Address Translation and Protection Register
#[derive(Csr)]
pub struct Satp {
    mode: BitField<Atp, 60, 63>,
    asid: BitField<Hex, 44, 59>,
    ppn: BitField<Hex, 0, 43>,
}

/// Supervisor Trap Vector Base Address Register
#[derive(Csr)]
pub struct Stvec {
    base: BitField<Hex, 2, 63>,
    mode: BitField<Tvec, 0, 1>,
}

/// Supervisor Scratch Register
#[derive(Csr)]
pub struct Sscratch {
    sscratch: BitField<Hex, 0, 63>,
}

/// Supervisor Exception Program Counter
#[derive(Csr)]
pub struct Sepc {
    sepc: BitField<Hex, 0, 63>,
}

/// Supervisor trap Cause Register
#[derive(Csr)]
pub struct Scause {
    interrupt: BitField<Bool, 63, 63>,
    // FIXME: exeption code is scause[0,62] but for formatting purposes, the interrupt flag (scause[63]) is included into this bitfield as well
    code: BitField<ExcCode, 0, 63>,
}

/// Supervisor Counter-Enable Register
#[derive(Csr)]
pub struct Scounteren {
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
