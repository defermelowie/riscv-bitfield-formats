//! Allow formatting of objects that implement the [BitFieldFormat](Csr) trait

use std::fmt::Display;

/// Defines functions required for creating formattable fields
pub trait Csr // TODO: rename to BitFieldFormat?
where
    Self: Display,
{
    /// Create a new instance from a value
    fn new(value: u64) -> Self
    where
        Self: Sized;

    // /// Create a boxed instance from a value
    // fn boxed_new(value: u64) -> Box<Self>
    // where
    //     Self: Sized;

    /// Get its name
    fn name() -> String
    where
        Self: Sized;
}
