//! Instruction formats

use thiserror::Error;

use crate::format::Csr;

mod encoding_variants;
pub use encoding_variants::*;
mod base_opcodes;
pub use base_opcodes::Opcode;
mod instructions;

/// Errors that may arise when formatting/handling instructions
#[derive(Error, Debug)]
pub enum InstrError {
    #[error("0x{0:016x} is not a known instruction")]
    UnknownInstruction(u64),
    #[error("0x{0:02x} is not a known opcode")]
    UnknownOpcode(u64),
    #[error("\"{0}\" is not a recognized as an instruction format")]
    NoInstructionFormat(String),
}

pub fn format(instr_str: &str, value: u64) -> Result<Box<dyn Csr>, InstrError> {
    match instr_str {
        "opcode" => Ok(Box::new(Opcode::new(value))),
        "r_inst" | "inst_r" => Ok(Box::new(RTypeInst::new(value))),
        "i_inst" | "inst_i" => Ok(Box::new(ITypeInst::new(value))),
        "s_inst" | "inst_s" => Ok(Box::new(STypeInst::new(value))),
        "b_inst" | "inst_b" => Ok(Box::new(BTypeInst::new(value))),
        "u_inst" | "inst_u" => Ok(Box::new(UTypeInst::new(value))),
        "j_inst" | "inst_j" => Ok(Box::new(JTypeInst::new(value))),
        "ins" | "inst" | "instr" => instructions::format_instr(value),
        _ => Err(InstrError::NoInstructionFormat(instr_str.into())),
    }
}
