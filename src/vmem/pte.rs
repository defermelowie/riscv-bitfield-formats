//! Page table entry formats

use csr_macro::Csr;
use std::fmt::Display;

use crate::bitfield::{Bin, BitField, Ppn};
use crate::format::Csr;

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
