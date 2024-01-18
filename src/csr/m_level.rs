//! Definitions for the machine mode CSRs
use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::{Arch, Bin, Hex, Priv, Tvec};
use crate::bitfield::{BitField, RSh};

/// Machine ISA Register
#[derive(Csr)]
pub struct Misa {
    a: BitField<Bin, 0, 0>,
    b: BitField<Bin, 1, 1>,
    c: BitField<Bin, 2, 2>,
    d: BitField<Bin, 3, 3>,
    e: BitField<Bin, 4, 4>,
    f: BitField<Bin, 5, 5>,
    g: BitField<Bin, 6, 6>,
    h: BitField<Bin, 7, 7>,
    i: BitField<Bin, 8, 8>,
    j: BitField<Bin, 9, 9>,
    k: BitField<Bin, 10, 10>,
    l: BitField<Bin, 11, 11>,
    m: BitField<Bin, 12, 12>,
    n: BitField<Bin, 13, 13>,
    o: BitField<Bin, 14, 14>,
    p: BitField<Bin, 15, 15>,
    q: BitField<Bin, 16, 16>,
    r: BitField<Bin, 17, 17>,
    s: BitField<Bin, 18, 18>,
    t: BitField<Bin, 19, 19>,
    u: BitField<Bin, 20, 20>,
    v: BitField<Bin, 21, 21>,
    w: BitField<Bin, 22, 22>,
    x: BitField<Bin, 23, 23>,
    y: BitField<Bin, 24, 24>,
    z: BitField<Bin, 25, 25>,
    mxl: BitField<Arch, 62, 63>,
}

/// Machine Vendor ID Register
#[derive(Csr)]
pub struct Mvendorid {
    offset: BitField<Hex, 0, 6>,
    bank: BitField<Hex, 7, 31>,
}

/// Machine Architecture ID Register
#[derive(Csr)]
pub struct Marchid {
    id: BitField<Hex, 0, 63>,
}

/// Machine Implementation ID Register
#[derive(Csr)]
pub struct Mimpid {
    id: BitField<Hex, 0, 63>,
}

/// Hart ID Register                         
#[derive(Csr)]
pub struct Mhartid {
    id: BitField<Hex, 0, 63>,
}

/// Machine Status Register                  
#[derive(Csr)]
pub struct Mstatus {
    sie: BitField<Bin, 1, 1>,
    mie: BitField<Bin, 3, 3>,
    spie: BitField<Bin, 5, 5>,
    ube: BitField<Bin, 6, 6>,
    mpie: BitField<Bin, 7, 7>,
    spp: BitField<Priv, 8, 8>,
    vs: BitField<Bin, 9, 10>,
    mpp: BitField<Priv, 11, 12>,
    fs: BitField<Bin, 13, 14>,
    xs: BitField<Bin, 15, 16>,
    mprv: BitField<Bin, 17, 17>,
    sum: BitField<Bin, 18, 18>,
    mxr: BitField<Bin, 19, 19>,
    tvm: BitField<Bin, 20, 20>,
    tw: BitField<Bin, 21, 21>,
    tsr: BitField<Bin, 22, 22>,
    uxl: BitField<Arch, 32, 33>,
    sxl: BitField<Arch, 34, 35>,
    sbe: BitField<Bin, 36, 36>,
    mbe: BitField<Bin, 37, 37>,
    gva: BitField<Bin, 38, 38>,
    mpv: BitField<Bin, 39, 39>,
    sd: BitField<Bin, 63, 63>,
}

/// Machine Trap-Vector Base-Address Register
#[derive(Csr)]
pub struct Mtvec {
    base: BitField<Hex, 2, 63>,
    mode: BitField<Tvec, 0, 1>,
}

/// Machine Exception Delegation Register
#[derive(Csr)]
pub struct Medeleg {
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

/// Machine Trap Value Register
#[derive(Csr)]
pub struct Mtval {
    tval: BitField<Hex, 0, 63>,
}

/// Machine Trap Value Register
#[derive(Csr)]
pub struct Mtval2 {
    tval: BitField<RSh<2, Hex>, 0, 63>,
}

/// Machine Interrupt Bitmap
#[derive(Csr)]
pub struct Minterrupts {
    s_software_i: BitField<Bin, 1, 1>,
    vs_software_i: BitField<Bin, 2, 2>,
    m_software_i: BitField<Bin, 3, 3>,
    s_timer_i: BitField<Bin, 5, 5>,
    vs_timer_i: BitField<Bin, 6, 6>,
    m_timer_i: BitField<Bin, 7, 7>,
    s_external_i: BitField<Bin, 9, 9>,
    vs_external_i: BitField<Bin, 10, 10>,
    m_external_i: BitField<Bin, 11, 11>,
    sg_external_i: BitField<Bin, 12, 12>,
    custom_i: BitField<Hex, 16, 63>,
}

/// Machine Interrupt Delegation Register
pub type Mideleg = Minterrupts;

/// Machine Interrupt Enable Register
pub type Mie = Minterrupts;

/// Machine Interrupt Pending Register
pub type Mip = Minterrupts;
