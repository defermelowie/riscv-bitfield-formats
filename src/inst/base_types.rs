//! Definitions for base instruction formats
//!
use csr_macro::Csr;
use std::fmt::Display;

use crate::format::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Bin, Dec, Hex, RSh};

#[derive(Csr)]
pub struct RTypeInst {
    opcode: BitField<Bin, 0, 6>,
    rd: BitField<Dec, 7, 11>,
    func3: BitField<Bin, 12, 14>,
    rs1: BitField<Dec, 15, 19>,
    rs2: BitField<Dec, 20, 24>,
    func7: BitField<Bin, 25, 31>,
}

#[derive(Csr)]
pub struct ITypeInst {
    opcode: BitField<Bin, 0, 6>,
    rd: BitField<Dec, 7, 11>,
    func3: BitField<Bin, 12, 14>,
    rs1: BitField<Dec, 15, 19>,
    imm: BitField<Hex, 20, 31>,
}

#[derive(Csr)]
pub struct STypeInst {
    // TODO: Create better way to display scattered immediate bits
    opcode: BitField<Bin, 0, 6>,
    imm0_4: BitField<Hex, 7, 11>,
    func3: BitField<Bin, 12, 14>,
    rs1: BitField<Dec, 15, 19>,
    rs2: BitField<Dec, 20, 24>,
    imm5_11:  BitField<RSh<5, Hex>, 25, 31>,
}

#[derive(Csr)]
pub struct BTypeInst {
    // TODO: Create better way to display scattered immediate bits
    opcode: BitField<Bin, 0, 6>,
    imm11: BitField<RSh<11, Hex>, 7, 7>,
    imm1_4: BitField<Hex, 8, 11>,
    func3: BitField<Bin, 12, 14>,
    rs1: BitField<Dec, 15, 19>,
    rs2: BitField<Dec, 20, 24>,
    imm5_10:  BitField<RSh<5, Hex>, 25, 30>,
    imm12:  BitField<RSh<12, Hex>, 31, 31>,
}

#[derive(Csr)]
pub struct UTypeInst {
    opcode: BitField<Bin, 0, 6>,
    rd: BitField<Dec, 7, 11>,
    imm: BitField<RSh<12, Hex>, 12, 31>
}

#[derive(Csr)]
pub struct JTypeInst {
    opcode: BitField<Bin, 0, 6>,
    rd: BitField<Dec, 7, 11>,
    imm12_19: BitField<RSh<12, Hex>, 12, 19>,
    imm11: BitField<RSh<11, Hex>, 20, 20>,
    imm1_10: BitField<RSh<1, Hex>, 21, 30>,
    imm20: BitField<RSh<20, Hex>, 31, 31>,
}
