// timepriv.rs
//
// Copyright (C) 2019  Minnesota Department of Transportation
//
//! Private module for time structs
//!
use crate::time::Unit;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

/// A measurement of _period_, _duration_ or _interval_ of time.
///
/// Period is a base quantity with a specific [unit].
///
/// ## Operations
///
/// * Period `+` Period `=>` Period
/// * Period `-` Period `=>` Period
/// * Period `*` f64 `=>` Period
/// * f64 `*` Period `=>` Period
/// * f64 `*` [unit] `=>` Period
/// * f64 `/` Period `=>` [Frequency]
///
/// Units must be the same for operations with two Period operands.  The [to]
/// method can be used for conversion.
///
/// ```rust
/// use mag::time::min;
///
/// let a = 15.0 * min;
/// let b = 5.5 * min;
///
/// assert_eq!(a.to_string(), "15 min");
/// assert_eq!((a + b).to_string(), "20.5 min");
/// ```
/// [Frequency]: struct.Frequency.html
/// [unit]: time/index.html
/// [to]: struct.Period.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Period<U> where U: Unit {
    /// Period quantity
    pub quantity: f64,
    /// Measurement unit
    unit: PhantomData<U>,
}

/// A measurement of _temporal frequency_ for repeating events.
///
/// Frequency is a derived quantity with a specific [unit].
///
/// ## Operations
///
/// * Frequency `+` Frequency `=>` Frequency
/// * Frequency `-` Frequency `=>` Frequency
/// * Frequency `*` f64 `=>` Frequency
/// * f64 `*` Frequency `=>` Frequency
/// * f64 `/` [Period] `=>` Frequency
/// * f64 `/` [unit] `=>` Frequency
/// * f64 `/` Frequency `=>` [Period]
///
/// Units must be the same for operations with two Frequency operands.  The [to]
/// method can be used for conversion.
///
/// ```rust
/// use mag::time::s;
///
/// let a = 25.0 / s;
///
/// assert_eq!(a.to_string(), "25 Hz");
/// ```
/// [Period]: struct.Period.html
/// [unit]: time/index.html
/// [to]: struct.Frequency.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frequency<U> where U: Unit {
    /// Frequency quantity
    pub quantity: f64,
    /// Measurement unit
    unit: PhantomData<U>,
}

impl_base_ops!(Period, Unit);
impl_base_ops!(Frequency, Unit);

impl<U> fmt::Display for Period<U> where U: Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, U::ABBREVIATION)
    }
}

impl<U> fmt::Display for Frequency<U> where U: Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, U::INVERSE)
    }
}

impl<U> Period<U> where U: Unit {
    /// Create a new period measurement
    pub fn new(quantity: f64) -> Self {
        Period::<U> { quantity, unit: PhantomData }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Period<T> {
        let quantity = self.quantity * U::factor::<T>();
        Period::<T> { quantity, unit: PhantomData }
    }
}

impl<U> Div<Period<U>> for f64 where U: Unit {
    type Output = Frequency<U>;

    fn div(self, other: Period<U>) -> Self::Output {
        let quantity = self / other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}

impl<U> Frequency<U> where U: Unit {
    /// Create a new frequency measurement
    pub fn new(quantity: f64) -> Self {
        Frequency::<U> { quantity, unit: PhantomData }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Frequency<T> {
        let quantity = self.quantity * U::factor::<T>();
        Frequency::<T> { quantity, unit: PhantomData }
    }
}

impl<U> Div<Frequency<U>> for f64 where U: Unit {
    type Output = Period<U>;

    fn div(self, other: Frequency<U>) -> Self::Output {
        let quantity = self / other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}
