// masspriv.rs
//
// Copyright (C) 2021  Minnesota Department of Transportation
//
//! Private module for mass
//!
use crate::mass::Unit;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

/// Quantity of mass.
///
/// Mass is a base quantity with a specific [unit].
///
/// ## Operations
///
/// * f64 `*` [unit] `=>` Mass
/// * i32 `*` [unit] `=>` Mass
/// * Mass `+` Mass `=>` Mass
/// * Mass `-` Mass `=>` Mass
/// * Mass `*` f64 `=>` Mass
/// * f64 `*` Mass `=>` Mass
/// * Mass `/` f64 `=>` Mass
///
/// Units must be the same for operations with two Mass operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::{Mass, mass::kg};
///
/// let a = 2.5 * kg;
/// let b = 4.5 * kg;
///
/// assert_eq!(a.to_string(), "2.5 kg");
/// assert_eq!(a + b, 7 * kg);
/// ```
/// [unit]: mass/index.html
/// [to]: struct.Mass.html#method.to
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mass<U>
where
    U: Unit,
{
    /// Mass quantity
    pub quantity: f64,

    /// Measurement unit
    unit: PhantomData<U>,
}

impl_base_ops!(Mass, Unit);

impl<U> Mass<U>
where
    U: Unit,
{
    /// Create a new mass quantity
    pub fn new(quantity: f64) -> Self {
        Mass::<U> {
            quantity,
            unit: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Mass<T> {
        let quantity = self.quantity * U::factor::<T>();
        Mass::new(quantity)
    }
}

impl<U> fmt::Display for Mass<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}
