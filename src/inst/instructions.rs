use crate::format::Csr;

use super::InstrError;

/// Build a instruction [BitFieldStruct] from its name & value
pub(super) fn format_instr(value: u64) -> Result<Box<dyn Csr>, InstrError> {
    todo!("Format instructions")
}
