use csr_macro::Csr;
use std::fmt::Display;

use crate::format::Csr;
use crate::bitfield::{BitField, BitFieldType};

use super::InstrError;

/// An opcode is a single 7-bit field
#[derive(Csr)]
pub struct Opcode {
    opcode: BitField<OpcodeType, 0, 6>
}

/// Enum to represent opcodes from the base opcode map
#[derive(Debug)]
enum OpcodeType {
    Load = 0b00_000_11,
    // Load floating point
    LoadFp = 0b00_001_11,
    Custom0 = 0b00_010_11,
    MiscMem = 0b00_011_11,
    OpImm = 0b00_100_11,
    /// Add upper immediate to PC
    Auipc = 0b00_101_11,
    OpImm32 = 0b00_110_11,
    Store = 0b01_000_11,
    /// Store floating point
    StoreFp = 0b01_001_11,
    Custom1 = 0b01_010_11,
    Amo = 0b01_011_11,
    Op = 0b01_100_11,
    /// Load upper immediate
    Lui = 0b01_101_11,
    Op32 = 0b01_110_11,
    /// Multiply-add
    MAdd = 0b10_000_11,
    /// Multiply-subtract
    MSub = 0b10_001_11,
    /// Negative multiply-subtract
    NMSub = 0b10_010_11,
    /// Negative multiply-add
    NMAdd = 0b10_011_11,
    OpFp = 0b10_100_11,
    Reserved0 = 0b10_101_11,
    Custom2 = 0b10_110_11,
    Branch = 0b11_000_11,
    /// Jump and link register
    Jalr = 0b11_001_11,
    Reserved1 = 0b11_010_11,
    /// Jump and link
    Jal = 0b11_011_11,
    System = 0b11_100_11,
    Reserved2 = 0b11_101_11,
    Custom3 = 0b11_110_11,
    // TODO: compressed instructions
}

impl TryFrom<u64> for OpcodeType {
    type Error = super::InstrError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            v if v == OpcodeType::Load as u64 => Ok(OpcodeType::Load),
            v if v == OpcodeType::LoadFp as u64 => Ok(OpcodeType::LoadFp),
            v if v == OpcodeType::Custom0 as u64 => Ok(OpcodeType::Custom0),
            v if v == OpcodeType::MiscMem as u64 => Ok(OpcodeType::MiscMem),
            v if v == OpcodeType::OpImm as u64 => Ok(OpcodeType::OpImm),
            v if v == OpcodeType::Auipc as u64 => Ok(OpcodeType::Auipc),
            v if v == OpcodeType::OpImm32 as u64 => Ok(OpcodeType::OpImm32),
            v if v == OpcodeType::Store as u64 => Ok(OpcodeType::Store),
            v if v == OpcodeType::StoreFp as u64 => Ok(OpcodeType::StoreFp),
            v if v == OpcodeType::Custom1 as u64 => Ok(OpcodeType::Custom1),
            v if v == OpcodeType::Amo as u64 => Ok(OpcodeType::Amo),
            v if v == OpcodeType::Op as u64 => Ok(OpcodeType::Op),
            v if v == OpcodeType::Lui as u64 => Ok(OpcodeType::Lui),
            v if v == OpcodeType::Op32 as u64 => Ok(OpcodeType::Op32),
            v if v == OpcodeType::MAdd as u64 => Ok(OpcodeType::MAdd),
            v if v == OpcodeType::MSub as u64 => Ok(OpcodeType::MSub),
            v if v == OpcodeType::NMSub as u64 => Ok(OpcodeType::NMSub),
            v if v == OpcodeType::NMAdd as u64 => Ok(OpcodeType::NMAdd),
            v if v == OpcodeType::OpFp as u64 => Ok(OpcodeType::OpFp),
            v if v == OpcodeType::Reserved0 as u64 => Ok(OpcodeType::Reserved0),
            v if v == OpcodeType::Custom2 as u64 => Ok(OpcodeType::Custom2),
            v if v == OpcodeType::Branch as u64 => Ok(OpcodeType::Branch),
            v if v == OpcodeType::Jalr as u64 => Ok(OpcodeType::Jalr),
            v if v == OpcodeType::Reserved1 as u64 => Ok(OpcodeType::Reserved1),
            v if v == OpcodeType::Jal as u64 => Ok(OpcodeType::Jal),
            v if v == OpcodeType::System as u64 => Ok(OpcodeType::System),
            v if v == OpcodeType::Reserved2 as u64 => Ok(OpcodeType::Reserved2),
            v if v == OpcodeType::Custom3 as u64 => Ok(OpcodeType::Custom3),
            _ => Err(InstrError::UnknownOpcode(value)),
        }
    }
}

impl BitFieldType for OpcodeType {
    fn decode(value: u64, size: usize) -> String {
        let opcode: Result<OpcodeType, _> = value.try_into();
        if let Ok(o) = opcode {
            match o {
                OpcodeType::Load => "LOAD".into(),
                OpcodeType::LoadFp => "LOAD-FP".into(),
                OpcodeType::Custom0 => "custom-0".into(),
                OpcodeType::MiscMem => "MISC-MEM".into(),
                OpcodeType::OpImm => "OP-IMM".into(),
                OpcodeType::Auipc => "AUIPC".into(),
                OpcodeType::OpImm32 => "OP-IMM-32".into(),
                OpcodeType::Store => "STORE".into(),
                OpcodeType::StoreFp => "STORE-FP".into(),
                OpcodeType::Custom1 => "custom-1".into(),
                OpcodeType::Amo => "AMO".into(),
                OpcodeType::Op => "OP".into(),
                OpcodeType::Lui => "LUI".into(),
                OpcodeType::Op32 => "OP-32".into(),
                OpcodeType::MAdd => "MADD".into(),
                OpcodeType::MSub => "MSUB".into(),
                OpcodeType::NMSub => "NMSUB".into(),
                OpcodeType::NMAdd => "NMADD".into(),
                OpcodeType::OpFp => "OP-FP".into(),
                OpcodeType::Reserved0 => "reserved".into(),
                OpcodeType::Custom2 => "custom-2".into(),
                OpcodeType::Branch => "BRANCH".into(),
                OpcodeType::Jalr => "JALR".into(),
                OpcodeType::Reserved1 => "reserved".into(),
                OpcodeType::Jal => "JAL".into(),
                OpcodeType::System => "SYSTEM".into(),
                OpcodeType::Reserved2 => "reserved".into(),
                OpcodeType::Custom3 => "custom-3".into(),
            }
        } else {
            format!("\x1b[33mInvalid base opcode (0x{:02x})\x1b[0m", value)
        }
    }
}
