//! CSR definitions for the base ISA spec

use bits::{B1, B2};

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
        println!("");
        println!("MISA");
        println!("----");
        println!("a: {}", &self.a);
        println!("b: {}", &self.b);
        println!("c: {}", &self.c);
        println!("d: {}", &self.d);
        println!("e: {}", &self.e);
        println!("f: {}", &self.f);
        println!("g: {}", &self.g);
        println!("h: {}", &self.h);
        println!("i: {}", &self.i);
        println!("j: {}", &self.j);
        println!("k: {}", &self.k);
        println!("l: {}", &self.l);
        println!("m: {}", &self.m);
        println!("n: {}", &self.n);
        println!("o: {}", &self.o);
        println!("p: {}", &self.p);
        println!("q: {}", &self.q);
        println!("r: {}", &self.r);
        println!("s: {}", &self.s);
        println!("t: {}", &self.t);
        println!("u: {}", &self.u);
        println!("v: {}", &self.v);
        println!("w: {}", &self.w);
        println!("x: {}", &self.x);
        println!("y: {}", &self.y);
        println!("z: {}", &self.z);
        println!("mxl: {}", &self.mxl);
    }
}
