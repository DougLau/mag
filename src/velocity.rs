// velocity.rs
//
// Copyright (C) 2019  Minnesota Department of Transportation
//
//! Private module for velocity structs
//!
use crate::{length, time};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

/// A measurement of _velocity_ or _speed_.
///
/// Velocity is a derived quantity with [length unit]s and [time unit]s.
///
/// ## Operations
///
/// * Velocity `+` Velocity `=>` Velocity
/// * Velocity `-` Velocity `=>` Velocity
/// * Velocity `*` f64 `=>` Velocity
/// * f64 `*` Velocity `=>` Velocity
/// * [Length] `*` [Frequency] `=>` Velocity
/// * Velocity `/` f64 `=>` Velocity
/// * [Length] `/` [time unit] `=>` Velocity
/// * [Length] `/` [Period] `=>` Velocity
///
/// Units must be the same for operations with two Velocity operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::{Velocity, length::{m, mi}, time::{h, s}};
///
/// let a = 7.4 * m / s;
/// let b = 55.0 * mi / h;
///
/// assert_eq!(a.to_string(), "7.4 m/s");
/// assert_eq!(b.to_string(), "55 mi/h");
/// ```
/// [Frequency]: struct.Frequency.html
/// [Length]: struct.Length.html
/// [Period]: struct.Period.html
/// [length unit]: length/index.html
/// [time unit]: time/index.html
/// [to]: struct.Velocity.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Velocity<L, P> where L: length::Unit, P: time::Unit {
    /// Velocity quantity
    pub quantity: f64,
    /// Length unit
    length: PhantomData<L>,
    /// Period unit
    period: PhantomData<P>,
}

impl<L, P> Add for Velocity<L, P> where L: length::Unit, P: time::Unit {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let quantity = self.quantity + other.quantity;
        Self { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> Sub for Velocity<L, P> where L: length::Unit, P: time::Unit {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let quantity = self.quantity - other.quantity;
        Self { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> Mul<f64> for Velocity<L, P> where L: length::Unit, P: time::Unit {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        let quantity = self.quantity * other;
        Self { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> Mul<Velocity<L, P>> for f64 where L: length::Unit, P: time::Unit {
    type Output = Velocity<L, P>;

    fn mul(self, other: Velocity<L, P>) -> Self::Output {
        let quantity = self * other.quantity;
        Velocity { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> Div<f64> for Velocity<L, P> where L: length::Unit, P: time::Unit {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        let quantity = self.quantity / other;
        Self { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> Velocity<L, P> where L: length::Unit, P: time::Unit {
    /// Create a new length measurement
    pub fn new(quantity: f64) -> Self {
        Velocity::<L, P> {
            quantity,
            length: PhantomData,
            period: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<N, R>(self) -> Velocity<N, R>
        where N: length::Unit, R: time::Unit
    {
        let factor = L::factor::<N>() / P::factor::<R>();
        let quantity = self.quantity * factor;
        Velocity { quantity, length: PhantomData, period: PhantomData }
    }
}

impl<L, P> fmt::Display for Velocity<L, P>
    where L: length::Unit, P: time::Unit
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}/{}", self.quantity, L::ABBREVIATION, P::ABBREVIATION)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::length::*;
    use super::super::time::*;

    #[test]
    fn vel_display() {
        assert_eq!((23.4 * m / s).to_string(), "23.4 m/s");
        assert_eq!((45.55 * mi / h).to_string(), "45.55 mi/h");
        assert_eq!((25.1 * mm / d).to_string(), "25.1 mm/d");
    }

    #[test]
    fn vel_to() {
        assert_eq!((88.0 * ft / s).to(), 59.999999998752 * mi / h);
        assert_eq!((55.0 * mi / h).to(), 88.51392000000001 * km / h);
    }

    #[test]
    fn vel_add() {
        assert_eq!(10.1 * nm / s + 15.1 * nm / s, 25.2 * nm / s);
        assert_eq!(20. * km / h + 30. * km / h, 50.0 * km / h);
    }

    #[test]
    fn vel_sub() {
        assert_eq!(55.6 * mm / d - 33.0 * mm / d, 22.6 * mm / d);
        assert_eq!(10.0 * km / ms - 5.5 * km / ms, 4.5 * km / ms);
    }

    #[test]
    fn vel_mul() {
        assert_eq!((5.1 * In / s) * 2.0, 10.2 * In / s);
        assert_eq!(3.0 * (10.5 * mi / us), 31.5 * mi / us);
        // Length * Frequency => Velocity
        assert_eq!((15.0 * m) * (3.0 / ds), 45.0 * m / ds);
        // Frequency * Length => Velocity
        assert_eq!((5.0 / s) * (3.0 * yd), 15.0 * yd / s);
    }

    #[test]
    fn time_div() {
        // Length / [time unit] => Velocity
        assert_eq!(10.0 * mi / h, Velocity::<mi, h>::new(10.0));
        // Length / Period => Velocity
        assert_eq!((45.5 * km) / (1.0 * h), Velocity::<km, h>::new(45.5));
    }
}
