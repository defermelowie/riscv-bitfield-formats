//! Instruction formats

use thiserror::Error;

use crate::bitfield::{Bin, BitField};
use crate::format::Csr;

mod base_types;
pub use base_types::*;

/// Errors that may arise when formatting/handling instructions
#[derive(Error, Debug)]
pub enum InstrError {
    #[error("0x{0:016x} is not a known instruction")]
    UnknownInstruction(u64),
}

/// Build a instruction [BitFieldStruct] from its name & value
pub fn format_instr(value: u64) -> Result<Box<dyn Csr>, InstrError> {
    let opcode: BitField<Bin, 0, 6> = value.into();
    let opcode = match opcode.value() {
        0b00_000_11 => Ok("LOAD"),
        0b00_001_11 => Ok("LOAD-FP"),
        0b00_011_11 => Ok("MISC-MEM"),
        0b00_100_11 => Ok("OP-IMM"),
        0b00_101_11 => Ok("AUIPC"),
        0b00_110_11 => Ok("OP-IMM-32"),

        0b01_000_11 => Ok("STORE"),

        0b10_000_11 => Ok("MADD"),

        0b11_000_11 => Ok("BRANCH"),

        _ => Err(InstrError::UnknownInstruction(value))
    };
    println!("Found opcode: {}", opcode.unwrap_or("Unknown"));
    todo!("Format instructions")
}

pub fn format(instr_str: &str, value: u64) -> Result<Box<dyn Csr>, InstrError> {
    match instr_str {
        "opcode" => todo!(),
        "rtype" => Ok(Box::new(RTypeInst::new(value))),
        "itype" => Ok(Box::new(ITypeInst::new(value))),
        "stype" => Ok(Box::new(STypeInst::new(value))),
        "btype" => Ok(Box::new(BTypeInst::new(value))),
        "utype" => Ok(Box::new(UTypeInst::new(value))),
        "jtype" => Ok(Box::new(JTypeInst::new(value))),
        _ => Err(InstrError::UnknownInstruction(value))
    }
}
