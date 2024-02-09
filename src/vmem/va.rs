//! virtual address formats

use csr_macro::Csr;
use std::fmt::Display;

use crate::bitfield::{BitField, Hex, RSh};
use crate::format::Csr;

/// Sv32 Virtual Address
#[derive(Csr)]
pub struct VAddr32 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<RSh<2, Hex>, 12, 21>,
    vpn1: BitField<RSh<2, Hex>, 22, 31>,
}

/// Sv39 Virtual Address
#[derive(Csr)]
pub struct VAddr39 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<RSh<3, Hex>, 12, 20>,
    vpn1: BitField<RSh<3, Hex>, 21, 29>,
    vpn2: BitField<RSh<3, Hex>, 30, 38>,
}

/// Sv48 Virtual Address
#[derive(Csr)]
pub struct VAddr48 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<RSh<3, Hex>, 12, 20>,
    vpn1: BitField<RSh<3, Hex>, 21, 29>,
    vpn2: BitField<RSh<3, Hex>, 30, 38>,
    vpn3: BitField<RSh<3, Hex>, 39, 47>,
}

/// Sv57 Virtual Address
#[derive(Csr)]
pub struct VAddr57 {
    page_offset: BitField<Hex, 0, 11>,
    vpn0: BitField<RSh<3, Hex>, 12, 20>,
    vpn1: BitField<RSh<3, Hex>, 21, 29>,
    vpn2: BitField<RSh<3, Hex>, 30, 38>,
    vpn3: BitField<RSh<3, Hex>, 39, 47>,
    vpn4: BitField<RSh<3, Hex>, 48, 56>,
}
