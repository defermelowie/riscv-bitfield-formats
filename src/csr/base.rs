//! CSR definitions for the base ISA spec

use bits::{Bits, B1, B16, B2, B25, B4, B44, B64, B7};

use super::{get_bit, get_bits, Csr};

/**************************************************************/
/* Machine ISA Register                                       */

pub struct Misa {
    a: B1,
    b: B1,
    c: B1,
    d: B1,
    e: B1,
    f: B1,
    g: B1,
    h: B1,
    i: B1,
    j: B1,
    k: B1,
    l: B1,
    m: B1,
    n: B1,
    o: B1,
    p: B1,
    q: B1,
    r: B1,
    s: B1,
    t: B1,
    u: B1,
    v: B1,
    w: B1,
    x: B1,
    y: B1,
    z: B1,
    mxl: B2,
}

impl Csr for Misa {
    fn new(value: u64) -> Self
    where
        Self: Sized,
    {
        Misa {
            a: B1(get_bit(value, 0)),
            b: B1(get_bit(value, 1)),
            c: B1(get_bit(value, 2)),
            d: B1(get_bit(value, 3)),
            e: B1(get_bit(value, 4)),
            f: B1(get_bit(value, 5)),
            g: B1(get_bit(value, 6)),
            h: B1(get_bit(value, 7)),
            i: B1(get_bit(value, 8)),
            j: B1(get_bit(value, 9)),
            k: B1(get_bit(value, 10)),
            l: B1(get_bit(value, 11)),
            m: B1(get_bit(value, 12)),
            n: B1(get_bit(value, 13)),
            o: B1(get_bit(value, 14)),
            p: B1(get_bit(value, 15)),
            q: B1(get_bit(value, 16)),
            r: B1(get_bit(value, 17)),
            s: B1(get_bit(value, 18)),
            t: B1(get_bit(value, 19)),
            u: B1(get_bit(value, 20)),
            v: B1(get_bit(value, 21)),
            w: B1(get_bit(value, 22)),
            x: B1(get_bit(value, 23)),
            y: B1(get_bit(value, 24)),
            z: B1(get_bit(value, 25)),
            mxl: B2(get_bits(value, 62, 63)),
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
        println!("MXL: {}", dec_arch(self.mxl.to_u64()));
    }
}

/**************************************************************/
/* Machine Vendor ID Register                                 */

pub struct Mvendorid {
    offset: B7,
    bank: B25,
}

impl Csr for Mvendorid {
    fn new(v: u64) -> Self {
        Mvendorid {
            offset: B7(get_bits(v, 0, 6)),
            bank: B25(get_bits(v, 7, 31)),
        }
    }

    fn print(&self) {
        println!("mvendorid");
        println!("---------");
        println!("offset: 0x{:x}", &self.offset.to_u64());
        println!("bank: 0x{:x}", &self.bank.to_u64());
    }
}

/**************************************************************/
/* Machine Architecture ID Register                           */

pub struct Marchid {
    id: B64,
}

impl Csr for Marchid {
    fn new(v: u64) -> Self {
        Marchid { id: B64(v) }
    }

    fn print(&self) {
        println!("Architecture ID: 0x{:x}", &self.id.to_u64());
    }
}

/**************************************************************/
/* Machine Implementation ID Register                         */

pub struct Mimpid {
    id: B64,
}

impl Csr for Mimpid {
    fn new(v: u64) -> Self {
        Mimpid { id: B64(v) }
    }

    fn print(&self) {
        println!("Implementation ID: 0x{:x}", &self.id.to_u64());
    }
}

/**************************************************************/
/* Hart ID Register                                           */

pub struct Mhartid {
    id: B64,
}

impl Csr for Mhartid {
    fn new(v: u64) -> Self {
        Mhartid { id: B64(v) }
    }

    fn print(&self) {
        println!("");
        println!("Hart ID: 0x{:x}", &self.id.to_u64());
    }
}

/**************************************************************/
/* Machine Status Register                                    */

pub struct Mstatus {
    sie: B1,
    mie: B1,
    spie: B1,
    ube: B1,
    mpie: B1,
    spp: B1,
    vs: B2,
    mpp: B2,
    fs: B2,
    xs: B2,
    mprv: B1,
    sum: B1,
    mxr: B1,
    tvm: B1,
    tw: B1,
    tsr: B1,
    uxl: B2,
    sxl: B2,
    sbe: B1,
    mbe: B1,
    gva: B1,
    mpv: B1,
    sd: B1,
}

impl Csr for Mstatus {
    fn new(value: u64) -> Self {
        Mstatus {
            sie: B1(get_bit(value, 1)),
            mie: B1(get_bit(value, 3)),
            spie: B1(get_bit(value, 5)),
            ube: B1(get_bit(value, 6)),
            mpie: B1(get_bit(value, 7)),
            spp: B1(get_bit(value, 8)),
            vs: B2(get_bits(value, 9, 10)),
            mpp: B2(get_bits(value, 11, 12)),
            fs: B2(get_bits(value, 13, 14)),
            xs: B2(get_bits(value, 15, 16)),
            mprv: B1(get_bit(value, 17)),
            sum: B1(get_bit(value, 18)),
            mxr: B1(get_bit(value, 19)),
            tvm: B1(get_bit(value, 20)),
            tw: B1(get_bit(value, 21)),
            tsr: B1(get_bit(value, 22)),
            uxl: B2(get_bits(value, 32, 33)),
            sxl: B2(get_bits(value, 34, 35)),
            sbe: B1(get_bit(value, 36)),
            mbe: B1(get_bit(value, 37)),
            gva: B1(get_bit(value, 38)),
            mpv: B1(get_bit(value, 39)),
            sd: B1(get_bit(value, 63)),
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
        println!("SPP: {}", dec_priv(self.spp.to_u64()));
        println!("VS: {}", &self.vs);
        println!("MPP: {}", dec_priv(self.mpp.to_u64()));
        println!("FS: {}", &self.fs);
        println!("XS: {}", &self.xs);
        println!("MPRV: {}", &self.mprv);
        println!("SUM: {}", &self.sum);
        println!("MXR: {}", &self.mxr);
        println!("TVM: {}", &self.tvm);
        println!("TW: {}", &self.tw);
        println!("TSR: {}", &self.tsr);
        println!("UXL: {}", dec_arch(self.uxl.to_u64()));
        println!("SXL: {}", dec_arch(self.sxl.to_u64()));
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
    mode: B4,
    asid: B16,
    ppn: B44,
}

impl Csr for Satp {
    fn new(value: u64) -> Self {
        Satp {
            mode: B4(get_bits(value, 60, 63)),
            asid: B16(get_bits(value, 44, 59)),
            ppn: B44(get_bits(value, 0, 43)),
        }
    }

    fn print(&self) {
        println!("");
        println!("satp");
        println!("----");
        println!("MODE: {}", dec_at_mode(self.mode.to_u64()));
        println!("ASID: 0x{:x}", &self.asid.to_u64());
        println!("PPN: 0x{:x}", &self.ppn.to_u64());
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
        n => format!("Invalid architecture encoding (0b{:b})", n),
    }
}

/// Decode privilege level
pub fn dec_priv(privilege: u64) -> String {
    match privilege {
        0b00 => "User".into(),
        0b01 => "Supervisor".into(),
        0b11 => "Machine".into(),
        n => format!("Invalid privilege encoding (0b{:b})", n),
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
        n => format!("Invalid encoding (0b{:b})", n),
    }
}