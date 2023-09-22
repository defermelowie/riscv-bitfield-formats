//! Formats related to virtual memory
//!
//! *Note that these are not actual CSRs*

use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Bin, Hex, Ppn, Vpn};

/// Sv32 Virtual Address
#[derive(Csr)]
pub struct VAddr32 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<Vpn, 12, 21>,
    vpn1: BitField<Vpn, 22, 31>,
}

/// Sv39 Virtual Address
#[derive(Csr)]
pub struct VAddr39 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<Vpn, 12, 20>,
    vpn1: BitField<Vpn, 21, 29>,
    vpn2: BitField<Vpn, 30, 38>,
}

/// Sv48 Virtual Address
#[derive(Csr)]
pub struct VAddr48 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<Vpn, 12, 20>,
    vpn1: BitField<Vpn, 21, 29>,
    vpn2: BitField<Vpn, 30, 38>,
    vpn3: BitField<Vpn, 39, 47>,
}

/// Sv57 Virtual Address
#[derive(Csr)]
pub struct VAddr57 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<Vpn, 12, 20>,
    vpn1: BitField<Vpn, 21, 29>,
    vpn2: BitField<Vpn, 30, 38>,
    vpn3: BitField<Vpn, 39, 47>,
    vpn4: BitField<Vpn, 48, 56>,
}

/// Sv32 Physical Address
#[derive(Csr)]
pub struct PAddr32 {
    page_offset: BitField<Hex, 0, 11>,
    ppn: BitField<Ppn<32>, 12, 33>,
}

/// Sv39 Physical Address
#[derive(Csr)]
pub struct PAddr39 {
    page_offset: BitField<Hex, 0, 11>,
    ppn: BitField<Ppn<39>, 12, 55>,
}

/// Sv48 Physical Address
#[derive(Csr)]
pub struct PAddr48 {
    page_offset: BitField<Hex, 0, 11>,
    ppn: BitField<Ppn<48>, 12, 55>,
}

/// Sv57 Physical Address
#[derive(Csr)]
pub struct PAddr57 {
    page_offset: BitField<Hex, 0, 11>,
    ppn: BitField<Ppn<57>, 12, 55>,
}

/// Sv32 Page Table Entry
#[derive(Csr)]
pub struct Pte32 {
    valid: BitField<Bin, 0, 0>,
    read: BitField<Bin, 1, 1>,
    write: BitField<Bin, 2, 2>,
    exec: BitField<Bin, 3, 3>,
    user: BitField<Bin, 4, 4>,
    global: BitField<Bin, 5, 5>,
    accessed: BitField<Bin, 6, 6>,
    dirty: BitField<Bin, 7, 7>,
    rsw: BitField<Bin, 8, 9>,
    ppn: BitField<Ppn<32>, 10, 31>,
}

/// Sv39 Page Table Entry
#[derive(Csr)]
pub struct Pte39 {
    valid: BitField<Bin, 0, 0>,
    read: BitField<Bin, 1, 1>,
    write: BitField<Bin, 2, 2>,
    exec: BitField<Bin, 3, 3>,
    user: BitField<Bin, 4, 4>,
    global: BitField<Bin, 5, 5>,
    accessed: BitField<Bin, 6, 6>,
    dirty: BitField<Bin, 7, 7>,
    rsw: BitField<Bin, 8, 9>,
    ppn: BitField<Ppn<39>, 10, 53>,
    pbmt: BitField<Bin, 61, 62>,
    n: BitField<Bin, 63, 63>,
}

/// Sv48 Page Table Entry
#[derive(Csr)]
pub struct Pte48 {
    valid: BitField<Bin, 0, 0>,
    read: BitField<Bin, 1, 1>,
    write: BitField<Bin, 2, 2>,
    exec: BitField<Bin, 3, 3>,
    user: BitField<Bin, 4, 4>,
    global: BitField<Bin, 5, 5>,
    accessed: BitField<Bin, 6, 6>,
    dirty: BitField<Bin, 7, 7>,
    rsw: BitField<Bin, 8, 9>,
    ppn: BitField<Ppn<48>, 10, 53>,
    pbmt: BitField<Bin, 61, 62>,
    n: BitField<Bin, 63, 63>,
}

/// Sv57 Page Table Entry
#[derive(Csr)]
pub struct Pte57 {
    valid: BitField<Bin, 0, 0>,
    read: BitField<Bin, 1, 1>,
    write: BitField<Bin, 2, 2>,
    exec: BitField<Bin, 3, 3>,
    user: BitField<Bin, 4, 4>,
    global: BitField<Bin, 5, 5>,
    accessed: BitField<Bin, 6, 6>,
    dirty: BitField<Bin, 7, 7>,
    rsw: BitField<Bin, 8, 9>,
    ppn: BitField<Ppn<57>, 10, 53>,
    pbmt: BitField<Bin, 61, 62>,
    n: BitField<Bin, 63, 63>,
}