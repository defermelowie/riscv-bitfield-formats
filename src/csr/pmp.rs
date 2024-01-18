use csr_macro::Csr;
use std::fmt::Display;

use super::Csr;
use crate::bitfield::BitField;
use crate::bitfield::{Hex, PmpXCfg, RSh, Reserved};

/// Physical memory protection address register
#[derive(Csr)]
pub struct PmpAddr {
    reserved: BitField<Reserved<0, Hex>, 54, 63>,
    address: BitField<RSh<2, Hex>, 0, 53>,
}

/// Physical memory protection configuration register
#[derive(Csr)]
pub struct PmpCfg {
    pmp0cfg: BitField<PmpXCfg, 0, 7>,
    pmp1cfg: BitField<PmpXCfg, 8, 15>,
    pmp2cfg: BitField<PmpXCfg, 16, 23>,
    pmp3cfg: BitField<PmpXCfg, 24, 31>,
    pmp4cfg: BitField<PmpXCfg, 32, 39>,
    pmp5cfg: BitField<PmpXCfg, 40, 47>,
    pmp6cfg: BitField<PmpXCfg, 48, 55>,
    pmp7cfg: BitField<PmpXCfg, 56, 63>,
}
