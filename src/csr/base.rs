//! CSR definitions for the base ISA spec

use super::Csr;
use crate::bit::BitField;

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
    fn new(value: u64) -> Self
    {
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
        println!("A: {}", &self.a.value());
        println!("B: {}", &self.b.value());
        println!("C: {}", &self.c.value());
        println!("D: {}", &self.d.value());
        println!("E: {}", &self.e.value());
        println!("F: {}", &self.f.value());
        println!("G: {}", &self.g.value());
        println!("H: {}", &self.h.value());
        println!("I: {}", &self.i.value());
        println!("J: {}", &self.j.value());
        println!("K: {}", &self.k.value());
        println!("L: {}", &self.l.value());
        println!("M: {}", &self.m.value());
        println!("N: {}", &self.n.value());
        println!("O: {}", &self.o.value());
        println!("P: {}", &self.p.value());
        println!("Q: {}", &self.q.value());
        println!("R: {}", &self.r.value());
        println!("S: {}", &self.s.value());
        println!("T: {}", &self.t.value());
        println!("U: {}", &self.u.value());
        println!("V: {}", &self.v.value());
        println!("W: {}", &self.w.value());
        println!("X: {}", &self.x.value());
        println!("Y: {}", &self.y.value());
        println!("Z: {}", &self.z.value());
        println!("MXL: {}", dec_arch(self.mxl.value()));
    }
}

/**************************************************************/
/* Machine Vendor ID Register                                 */

pub struct Mvendorid {
    offset: BitField<0,6>,
    bank: BitField<7,31>,
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
        Mhartid { id: value.into()}
    }

    fn print(&self) {
        println!("");
        println!("Hart ID: 0x{:x}", &self.id.value());
    }
}

/**************************************************************/
/* Machine Status Register                                    */

// pub struct Mstatus {
//     sie: BitField,
//     mie: BitField,
//     spie: BitField,
//     ube: BitField,
//     mpie: BitField,
//     spp: BitField,
//     vs: B2,
//     mpp: B2,
//     fs: B2,
//     xs: B2,
//     mprv: BitField,
//     sum: BitField,
//     mxr: BitField,
//     tvm: BitField,
//     tw: BitField,
//     tsr: BitField,
//     uxl: B2,
//     sxl: B2,
//     sbe: BitField,
//     mbe: BitField,
//     gva: BitField,
//     mpv: BitField,
//     sd: BitField,
// }

// impl Csr for Mstatus {
//     fn new(value: u64) -> Self {
//         Mstatus {
//             sie: BitField(get_bit(value, 1)),
//             mie: BitField(get_bit(value, 3)),
//             spie: BitField(get_bit(value, 5)),
//             ube: BitField(get_bit(value, 6)),
//             mpie: BitField(get_bit(value, 7)),
//             spp: BitField(get_bit(value, 8)),
//             vs: B2(get_bits(value, 9, 10)),
//             mpp: B2(get_bits(value, 11, 12)),
//             fs: B2(get_bits(value, 13, 14)),
//             xs: B2(get_bits(value, 15, 16)),
//             mprv: BitField(get_bit(value, 17)),
//             sum: BitField(get_bit(value, 18)),
//             mxr: BitField(get_bit(value, 19)),
//             tvm: BitField(get_bit(value, 20)),
//             tw: BitField(get_bit(value, 21)),
//             tsr: BitField(get_bit(value, 22)),
//             uxl: B2(get_bits(value, 32, 33)),
//             sxl: B2(get_bits(value, 34, 35)),
//             sbe: BitField(get_bit(value, 36)),
//             mbe: BitField(get_bit(value, 37)),
//             gva: BitField(get_bit(value, 38)),
//             mpv: BitField(get_bit(value, 39)),
//             sd: BitField(get_bit(value, 63)),
//         }
//     }

//     fn print(&self) {
//         println!("mstatus");
//         println!("-------");
//         println!("SIE: {}", &self.sie);
//         println!("MIE: {}", &self.mie);
//         println!("SPIE: {}", &self.spie);
//         println!("UBE: {}", &self.ube);
//         println!("MPIE: {}", &self.mpie);
//         println!("SPP: {}", dec_priv(self.spp.value()));
//         println!("VS: {}", &self.vs);
//         println!("MPP: {}", dec_priv(self.mpp.value()));
//         println!("FS: {}", &self.fs);
//         println!("XS: {}", &self.xs);
//         println!("MPRV: {}", &self.mprv);
//         println!("SUM: {}", &self.sum);
//         println!("MXR: {}", &self.mxr);
//         println!("TVM: {}", &self.tvm);
//         println!("TW: {}", &self.tw);
//         println!("TSR: {}", &self.tsr);
//         println!("UXL: {}", dec_arch(self.uxl.value()));
//         println!("SXL: {}", dec_arch(self.sxl.value()));
//         println!("SBE: {}", &self.sbe);
//         println!("MBE: {}", &self.mbe);
//         println!("GVA: {}", &self.gva);
//         println!("MPV: {}", &self.mpv);
//         println!("SD: {}", &self.sd);
//     }
// }

// /**************************************************************/
// /* Supervisor Address Translation and Protection Register     */

// pub struct Satp {
//     mode: B4,
//     asid: B16,
//     ppn: B44,
// }

// impl Csr for Satp {
//     fn new(value: u64) -> Self {
//         Satp {
//             mode: B4(get_bits(value, 60, 63)),
//             asid: B16(get_bits(value, 44, 59)),
//             ppn: B44(get_bits(value, 0, 43)),
//         }
//     }

//     fn print(&self) {
//         println!("");
//         println!("satp");
//         println!("----");
//         println!("MODE: {}", dec_at_mode(self.mode.value()));
//         println!("ASID: 0x{:x}", &self.asid.value());
//         println!("PPN: 0x{:x}", &self.ppn.value());
//     }
// }

// /**************************************************************/
// /* Helper functions                                           */

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