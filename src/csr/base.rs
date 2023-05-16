//! CSR definitions for the base ISA spec

use super::Csr;
use crate::bitfield::BitField;

/**************************************************************/
/* Machine ISA Register                                       */

pub struct Misa {
    a: BitField<0, 0>,
    b: BitField<1, 1>,
    c: BitField<2, 2>,
    d: BitField<3, 3>,
    e: BitField<4, 4>,
    f: BitField<5, 5>,
    g: BitField<6, 6>,
    h: BitField<7, 7>,
    i: BitField<8, 8>,
    j: BitField<9, 9>,
    k: BitField<10, 10>,
    l: BitField<11, 11>,
    m: BitField<12, 12>,
    n: BitField<13, 13>,
    o: BitField<14, 14>,
    p: BitField<15, 15>,
    q: BitField<16, 16>,
    r: BitField<17, 17>,
    s: BitField<18, 18>,
    t: BitField<19, 19>,
    u: BitField<20, 20>,
    v: BitField<21, 21>,
    w: BitField<22, 22>,
    x: BitField<23, 23>,
    y: BitField<24, 24>,
    z: BitField<25, 25>,
    mxl: BitField<62, 63>,
}

impl Csr for Misa {
    fn new(value: u64) -> Self {
        Misa {
            a: value.into(),
            b: value.into(),
            c: value.into(),
            d: value.into(),
            e: value.into(),
            f: value.into(),
            g: value.into(),
            h: value.into(),
            i: value.into(),
            j: value.into(),
            k: value.into(),
            l: value.into(),
            m: value.into(),
            n: value.into(),
            o: value.into(),
            p: value.into(),
            q: value.into(),
            r: value.into(),
            s: value.into(),
            t: value.into(),
            u: value.into(),
            v: value.into(),
            w: value.into(),
            x: value.into(),
            y: value.into(),
            z: value.into(),
            mxl: value.into(),
        }
    }

    fn print(&self) {
        println!("misa");
        println!("----");
        println!("A: {}", &self.a);
        println!("B: {}", &self.b);
        println!("C: {}", &self.c);
        println!("D: {}", &self.d);
        println!("E: {}", &self.e);
        println!("F: {}", &self.f);
        println!("G: {}", &self.g);
        println!("H: {}", &self.h);
        println!("I: {}", &self.i);
        println!("J: {}", &self.j);
        println!("K: {}", &self.k);
        println!("L: {}", &self.l);
        println!("M: {}", &self.m);
        println!("N: {}", &self.n);
        println!("O: {}", &self.o);
        println!("P: {}", &self.p);
        println!("Q: {}", &self.q);
        println!("R: {}", &self.r);
        println!("S: {}", &self.s);
        println!("T: {}", &self.t);
        println!("U: {}", &self.u);
        println!("V: {}", &self.v);
        println!("W: {}", &self.w);
        println!("X: {}", &self.x);
        println!("Y: {}", &self.y);
        println!("Z: {}", &self.z);
        println!("MXL: {}", dec_arch(self.mxl.value()));
    }
}

/**************************************************************/
/* Machine Vendor ID Register                                 */

pub struct Mvendorid {
    offset: BitField<0, 6>,
    bank: BitField<7, 31>,
}

impl Csr for Mvendorid {
    fn new(value: u64) -> Self {
        Mvendorid {
            offset: value.into(),
            bank: value.into(),
        }
    }

    fn print(&self) {
        println!("mvendorid");
        println!("---------");
        println!("offset: 0x{:x}", &self.offset.value());
        println!("bank: 0x{:x}", &self.bank.value());
    }
}

/**************************************************************/
/* Machine Architecture ID Register                           */

pub struct Marchid {
    id: BitField<0, 63>,
}

impl Csr for Marchid {
    fn new(value: u64) -> Self {
        Marchid { id: value.into() }
    }

    fn print(&self) {
        println!("Architecture ID: 0x{:x}", &self.id.value());
    }
}

/**************************************************************/
/* Machine Implementation ID Register                         */

pub struct Mimpid {
    id: BitField<0, 63>,
}

impl Csr for Mimpid {
    fn new(value: u64) -> Self {
        Mimpid { id: value.into() }
    }

    fn print(&self) {
        println!("Implementation ID: 0x{:x}", &self.id.value());
    }
}

/**************************************************************/
/* Hart ID Register                                           */

pub struct Mhartid {
    id: BitField<0, 63>,
}

impl Csr for Mhartid {
    fn new(value: u64) -> Self {
        Mhartid { id: value.into() }
    }

    fn print(&self) {
        println!("");
        println!("Hart ID: 0x{:x}", &self.id.value());
    }
}

/**************************************************************/
/* Machine Status Register                                    */

pub struct Mstatus {
    sie: BitField<1, 1>,
    mie: BitField<3, 3>,
    spie: BitField<5, 5>,
    ube: BitField<6, 6>,
    mpie: BitField<7, 7>,
    spp: BitField<8, 8>,
    vs: BitField<9, 10>,
    mpp: BitField<11, 12>,
    fs: BitField<13, 14>,
    xs: BitField<15, 16>,
    mprv: BitField<17, 17>,
    sum: BitField<18, 18>,
    mxr: BitField<19, 19>,
    tvm: BitField<20, 20>,
    tw: BitField<21, 21>,
    tsr: BitField<22, 22>,
    uxl: BitField<32, 33>,
    sxl: BitField<34, 35>,
    sbe: BitField<36, 36>,
    mbe: BitField<37, 37>,
    gva: BitField<38, 38>,
    mpv: BitField<39, 39>,
    sd: BitField<63, 63>,
}

impl Csr for Mstatus {
    fn new(value: u64) -> Self {
        Mstatus {
            sie: value.into(),
            mie: value.into(),
            spie: value.into(),
            ube: value.into(),
            mpie: value.into(),
            spp: value.into(),
            vs: value.into(),
            mpp: value.into(),
            fs: value.into(),
            xs: value.into(),
            mprv: value.into(),
            sum: value.into(),
            mxr: value.into(),
            tvm: value.into(),
            tw: value.into(),
            tsr: value.into(),
            uxl: value.into(),
            sxl: value.into(),
            sbe: value.into(),
            mbe: value.into(),
            gva: value.into(),
            mpv: value.into(),
            sd: value.into(),
        }
    }

    fn print(&self) {
        println!("mstatus");
        println!("-------");
        println!("SIE: {}", &self.sie);
        println!("MIE: {}", &self.mie);
        println!("SPIE: {}", &self.spie);
        println!("UBE: {}", &self.ube);
        println!("MPIE: {}", &self.mpie);
        println!("SPP: {}", dec_priv(self.spp.value()));
        println!("VS: {}", &self.vs);
        println!("MPP: {}", dec_priv(self.mpp.value()));
        println!("FS: {}", &self.fs);
        println!("XS: {}", &self.xs);
        println!("MPRV: {}", &self.mprv);
        println!("SUM: {}", &self.sum);
        println!("MXR: {}", &self.mxr);
        println!("TVM: {}", &self.tvm);
        println!("TW: {}", &self.tw);
        println!("TSR: {}", &self.tsr);
        println!("UXL: {}", dec_arch(self.uxl.value()));
        println!("SXL: {}", dec_arch(self.sxl.value()));
        println!("SBE: {}", &self.sbe);
        println!("MBE: {}", &self.mbe);
        println!("GVA: {}", &self.gva);
        println!("MPV: {}", &self.mpv);
        println!("SD: {}", &self.sd);
    }
}

/**************************************************************/
/* Supervisor Address Translation and Protection Register     */
pub struct Satp {
    mode: BitField<60, 63>,
    asid: BitField<44, 59>,
    ppn: BitField<0, 43>,
}

impl Csr for Satp {
    fn new(value: u64) -> Self {
        Satp {
            mode: value.into(),
            asid: value.into(),
            ppn: value.into(),
        }
    }

    fn print(&self) {
        println!("");
        println!("satp");
        println!("----");
        println!("MODE: {}", dec_at_mode(self.mode.value()));
        println!("ASID: 0x{:x}", &self.asid.value());
        println!("PPN: 0x{:x}", &self.ppn.value());
    }
}

/**************************************************************/
/* Machine Trap-Vector Base-Address Register                  */

pub struct Mtvec {
    base: BitField<2, 63>,
    mode: BitField<0, 1>,
}

impl Csr for Mtvec {
    fn new(value: u64) -> Self {
        Mtvec {
            base: value.into(),
            mode: value.into(),
        }
    }

    fn print(&self) {
        println!("");
        println!("mtvec");
        println!("-----");
        println!("BASE: 0x{:x}", &self.base.value() << 2);
        println!("MODE: {}", dec_xtvec_mode(self.mode.value()));
    }
}

/**************************************************************/
/* Helper functions                                           */
/// Decode architecture
pub fn dec_arch(architecture: u64) -> String {
    match architecture {
        0b01 => "RV32".into(),
        0b10 => "RV64".into(),
        0b11 => "RV128".into(),
        n => format!("\x1b[33mInvalid architecture (0b{:b})\x1b[0m", n),
    }
}

/// Decode privilege level
pub fn dec_priv(privilege: u64) -> String {
    match privilege {
        0b00 => "User".into(),
        0b01 => "Supervisor".into(),
        0b11 => "Machine".into(),
        n => format!("\x1b[33mInvalid privilege (0b{:b})\x1b[0m", n),
    }
}

/// Decode address translation mode
pub fn dec_at_mode(mode: u64) -> String {
    match mode {
        0x0 => "Bare".into(),
        0x1 => "Sv32".into(),
        0x8 => "Sv39".into(),
        0x9 => "Sv48".into(),
        0xa => "Sv57".into(),
        n => format!(
            "\x1b[33mInvalid address translation mode (0b{:b})\x1b[0m",
            n
        ),
    }
}

// Decode xtvec mode field
pub fn dec_xtvec_mode(mode: u64) -> String {
    match mode {
        0x0 => "Direct".into(),
        0x1 => "Vectored".into(),
        n => format!(
            "\x1b[33mInvalid (0b{:b})\x1b[0m",
            n
        ),
    }
}
