//! Definitions for the machine mode CSRs
use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Arch, Bin, Hex, Priv, Atp, Tvec};

/**************************************************************/
/* Machine ISA Register                                       */

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

/**************************************************************/
/* Machine Vendor ID Register                                 */

#[derive(Csr)]
pub struct Mvendorid {
    offset: BitField<Hex, 0, 6>,
    bank: BitField<Hex, 7, 31>,
}

/**************************************************************/
/* Machine Architecture ID Register                           */

#[derive(Csr)]
pub struct Marchid {
    id: BitField<Hex, 0, 63>,
}

/**************************************************************/
/* Machine Implementation ID Register                         */

#[derive(Csr)]
pub struct Mimpid {
    id: BitField<Hex, 0, 63>,
}

/**************************************************************/
/* Hart ID Register                                           */

#[derive(Csr)]
pub struct Mhartid {
    id: BitField<Hex, 0, 63>,
}

/**************************************************************/
/* Machine Status Register                                    */

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

/**************************************************************/
/* Machine Trap-Vector Base-Address Register                  */

#[derive(Csr)]
pub struct Mtvec {
    base: BitField<Hex, 2, 63>,
    mode: BitField<Tvec, 0, 1>,
}
