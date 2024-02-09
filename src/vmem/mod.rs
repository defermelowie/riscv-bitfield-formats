//! Formats related to virtual memory
//!
//! *Note that these are not CSRs but rather (double)words in memory*

use thiserror::Error;

use crate::{csr, format::Csr};

// Export vmem formats
mod va;
pub use va::*;
mod pa;
pub use pa::*;
mod pte;
pub use pte::*;

/// Errors that may arise when creating/handling a CSR
#[derive(Error, Debug)]
pub enum VmemError {
    #[error("\"{0}\" is not a name of a supported virtual memory related format")]
    Unknown(String),
}

/// Build a vmem [BitFieldStruct] from its name & value
pub fn format(vmem_str: &str, value: u64) -> Result<Box<dyn Csr>, VmemError> {
    match vmem_str {
        "pte_sv32" | "sv32_pte" => Ok(Box::new(Pte32::new(value))),
        "pte_sv39" | "sv39_pte" => Ok(Box::new(Pte39::new(value))),
        "pte_sv48" | "sv48_pte" => Ok(Box::new(Pte48::new(value))),
        "pte_sv57" | "sv57_pte" => Ok(Box::new(Pte57::new(value))),
        "vaddr_sv32" | "sv32_vaddr" => Ok(Box::new(VAddr32::new(value))),
        "vaddr_sv39" | "sv39_vaddr" => Ok(Box::new(VAddr39::new(value))),
        "vaddr_sv48" | "sv48_vaddr" => Ok(Box::new(VAddr48::new(value))),
        "vaddr_sv57" | "sv57_vaddr" => Ok(Box::new(VAddr57::new(value))),
        "paddr_sv32" | "sv32_paddr" => Ok(Box::new(PAddr32::new(value))),
        "paddr_sv39" | "sv39_paddr" => Ok(Box::new(PAddr39::new(value))),
        "paddr_sv48" | "sv48_paddr" => Ok(Box::new(PAddr48::new(value))),
        "paddr_sv57" | "sv57_paddr" => Ok(Box::new(PAddr57::new(value))),
        _ => Err(VmemError::Unknown(vmem_str.to_string())),
    }
}
