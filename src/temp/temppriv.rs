// temppriv.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
//
//! Private module for temperature structs
//!
extern crate alloc;

use crate::temp::Unit;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, Sub};

/// Thermodynamic _temperature_.
///
/// Temperature is a base quantity with a specific [unit].
///
/// ## Operations
///
/// * f64 `*` [unit] `=>` Temperature
/// * i32 `*` [unit] `=>` Temperature
/// * Temperature `+` Temperature `=>` Temperature
/// * Temperature `-` Temperature `=>` Temperature
///
/// Units must be the same for operations with two Temperature operands.  The
/// [to] method can be used for conversion.
///
/// ```rust
/// use mag::temp::{DegC, DegF};
///
/// let a = 72.5 * DegF;
/// let b = 100 * DegC;
///
/// assert_eq!(a.to_string(), "72.5 °F");
/// assert_eq!(b.to_string(), "100 °C");
/// ```
/// [unit]: temp/index.html
/// [to]: struct.Temperature.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Temperature<U>
where
    U: Unit,
{
    /// Temperature quantity
    pub quantity: f64,

    /// Measurement unit
    unit: PhantomData<U>,
}

impl<U> fmt::Display for Temperature<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<U> Temperature<U>
where
    U: Unit,
{
    /// Create a new temperature measurement
    pub fn new(quantity: f64) -> Self {
        Temperature {
            quantity,
            unit: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Temperature<T> {
        let dk = (self.quantity - U::K_ZERO) * U::K_FACTOR;
        let quantity = dk / T::K_FACTOR + T::K_ZERO;
        Temperature::new(quantity)
    }
}

// Temperature + Temperature => Temperature
impl<U> Add for Temperature<U>
where
    U: Unit,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.quantity + other.quantity)
    }
}

// Temperature - Temperature => Temperature
impl<U> Sub for Temperature<U>
where
    U: Unit,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.quantity - other.quantity)
    }
}
