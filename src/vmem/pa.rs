//! Physical address formats

use csr_macro::Csr;
use std::fmt::Display;

use crate::bitfield::{BitField, Hex, Ppn};
use crate::format::Csr;

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
