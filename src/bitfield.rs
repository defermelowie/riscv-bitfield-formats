//! Defines a generic bitfield type
use std::{fmt::Display, mem::size_of, marker::PhantomData};

/// Binary
pub struct Bin;
impl BitFieldType for Bin {
    fn decode(value: u64, size: usize) -> String {
        let bit_str = format!("{:b}", value);
        let zeroes = "0".repeat(size - bit_str.len());
        format!("0b{}{}", zeroes, bit_str)
    }
}
/// Hexidecimal
pub struct Hex;
impl BitFieldType for Hex {
    fn decode(value: u64, _size: usize) -> String {
        format!("0x{:x}", value)
    }
}
/// Architecture
pub struct Arch;
impl BitFieldType for Arch {
    /// Decode architecture
    fn decode(value: u64, _size: usize) -> String {
        match value {
            0b01 => "RV32".into(),
            0b10 => "RV64".into(),
            0b11 => "RV128".into(),
            n => format!("\x1b[33mInvalid architecture (0b{:b})\x1b[0m", n),
        }
    }
}
/// Privilege level
pub struct Priv;
impl BitFieldType for Priv {
    /// Decode privilege level
    fn decode(value: u64, _size: usize) -> String {
        match value {
            0b00 => "User".into(),
            0b01 => "Supervisor".into(),
            0b11 => "Machine".into(),
            n => format!("\x1b[33mInvalid privilege (0b{:b})\x1b[0m", n),
        }
    }
}
/// Address translation & protection mode
pub struct Atp;
impl BitFieldType for Atp {
    /// Decode address translation mode
    fn decode(value: u64, _size: usize) -> String {
        match value {
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
}
/// Trap vector mode
pub struct Tvec;
impl BitFieldType for Tvec {
    /// Decode xtvec mode field
    fn decode(value: u64, _size: usize) -> String {
        match value {
            0x0 => "Direct".into(),
            0x1 => "Vectored".into(),
            n => format!("\x1b[33mInvalid (0b{:b})\x1b[0m", n),
        }
    }
}

/// Bitfield types only differ in the format they are printed
trait BitFieldType {
    fn decode(value: u64, size: usize) -> String;
}

/// A bitfield with 2 constant generics to indicate the position of it's first and last bit
///
/// Requires `START <= END`
///
/// Notes:
/// - Add `where {START <= END}` whenever arithmitic on constants in where clauses gets supported.
/// - Make `BitFieldType::Bin` default whenever const genereic defaults for complex types gets supported.
pub struct BitField<T, const START: usize, const END: usize>(u64, PhantomData<T>);

impl<T, const S: usize, const E: usize> BitField<T, S, E> {
    /// Create a new bitfield from a raw value
    ///
    /// Value is taken from bits `START` to `END`
    pub fn new(value: u64) -> Self {
        assert!(S <= E);
        if Self::size() == 1 {
            BitField(get_bit(value.into(), S), PhantomData)
        } else {
            BitField(get_bits(value.into(), S, E), PhantomData)
        }
    }

    /// Get bitfield's size in bits
    pub fn size() -> usize {
        E - S + 1
    }

    /// Set bitfield's value
    ///
    /// Requires `value` to fit in bitfield's size
    pub fn set_value(&mut self, value: u64) {
        assert!(value < (1 << Self::size()));
        self.0 = value;
    }

    /// Get bitfield's value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<I, T, const S: usize, const E: usize> From<I> for BitField<T, S, E>
where
    I: Into<u64>,
{
    /// Converts any integer value into a bitfield
    fn from(value: I) -> Self {
        BitField::new(value.into())
    }
}

impl<T: BitFieldType, const S: usize, const E: usize> Display for BitField<T, S, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", T::decode(self.value(), Self::size()))
    }
}

/// Get the bit at a specified index
fn get_bit<I>(value: I, index: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= index);

    let value = value << (lastbit - index);
    let value = value >> (lastbit);
    value
}

/// Get the bits between 2 specified indices
fn get_bits<I>(value: I, start: usize, end: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= end);
    assert!(end >= start);

    let value = value << (lastbit - end);
    let value = value >> (lastbit - end + start);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn bitfield_start_after_end_panics() {
        let _: BitField<Bin, 7, 4> = BitField::new(3);
    }

    #[test]
    fn bitfield_size() {
        let s = BitField::<Bin, 4, 7>::size();
        assert_eq!(s, 4);
    }

    #[test]
    fn bitfield_from_u64() {
        let v = 0x4756_u64;
        let b: BitField<Bin, 2, 5> = v.into();
        assert_eq!(b.value(), 5);
    }

    #[test]
    fn bitfield_format_bin() {
        let b = BitField::<Bin, 0, 5>::new(0x5);
        let s = format!("{}", b);
        assert_eq!(s, "0b000101");
    }
    #[test]
    fn get_bit_u8() {
        assert_eq!(1, get_bit(0b0000_0010_u8, 1));
        assert_eq!(0, get_bit(0b1111_1011_u8, 2));
    }

    #[test]
    fn get_bit_u16() {
        assert_eq!(1, get_bit(0x4000_u16, 14));
        assert_eq!(0, get_bit(0xfdff_u16, 9));
    }

    #[test]
    fn get_bits_u16() {
        assert_eq!(3, get_bits(0b000_1100_u8, 2, 3));
        assert_eq!(0xf, get_bits(0b011_1100_u8, 2, 5));
    }
}
