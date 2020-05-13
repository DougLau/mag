// speed.rs
//
// Copyright (C) 2019-2020  Minnesota Department of Transportation
//
//! Private module for speed structs
//!
use crate::{length, time};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

/// A measurement of _speed_.
///
/// Speed is a derived quantity with [length unit]s and [time unit]s.
///
/// ## Operations
///
/// * Speed `+` Speed `=>` Speed
/// * Speed `-` Speed `=>` Speed
/// * Speed `*` f64 `=>` Speed
/// * f64 `*` Speed `=>` Speed
/// * [Length] `*` [Frequency] `=>` Speed
/// * Speed `/` f64 `=>` Speed
/// * [Length] `/` [time unit] `=>` Speed
/// * [Length] `/` [Period] `=>` Speed
///
/// Units must be the same for operations with two Speed operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::{Speed, length::{m, mi}, time::{h, s}};
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
/// [to]: struct.Speed.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    /// Speed quantity
    pub quantity: f64,
    /// Length unit
    length: PhantomData<L>,
    /// Period unit
    period: PhantomData<P>,
}

// Speed + Speed => Speed
impl<L, P> Add for Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.quantity + other.quantity)
    }
}

// Speed - Speed => Speed
impl<L, P> Sub for Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.quantity - other.quantity)
    }
}

// Speed * f64 => Speed
impl<L, P> Mul<f64> for Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.quantity * scalar)
    }
}

// f64 * Speed => Speed
impl<L, P> Mul<Speed<L, P>> for f64
where
    L: length::Unit,
    P: time::Unit,
{
    type Output = Speed<L, P>;
    fn mul(self, other: Speed<L, P>) -> Self::Output {
        Speed::new(self * other.quantity)
    }
}

// Speed / f64 => Speed
impl<L, P> Div<f64> for Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.quantity / scalar)
    }
}

impl<L, P> Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    /// Create a new length measurement
    pub fn new(quantity: f64) -> Self {
        Speed::<L, P> {
            quantity,
            length: PhantomData,
            period: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<N, R>(self) -> Speed<N, R>
    where
        N: length::Unit,
        R: time::Unit,
    {
        let factor = L::factor::<N>() / P::factor::<R>();
        Speed::new(self.quantity * factor)
    }
}

impl<L, P> fmt::Display for Speed<L, P>
where
    L: length::Unit,
    P: time::Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}/{}", L::ABBREVIATION, P::ABBREVIATION)
    }
}

#[cfg(test)]
mod test {
    use super::super::length::*;
    use super::super::time::*;
    use super::*;

    #[test]
    fn speed_display() {
        assert_eq!((23.4 * m / s).to_string(), "23.4 m/s");
        assert_eq!((45.55 * mi / h).to_string(), "45.55 mi/h");
        assert_eq!((25.1 * mm / d).to_string(), "25.1 mm/d");
        assert_eq!(format!("{:.0}", (88.0 * ft / s).to::<mi, h>()), "60 mi/h");
    }

    #[test]
    fn speed_to() {
        assert_eq!((88.0 * ft / s).to(), 59.999999998752 * mi / h);
        assert_eq!((55.0 * mi / h).to(), 88.51392000000001 * km / h);
    }

    #[test]
    fn speed_add() {
        assert_eq!(10.1 * nm / s + 15.1 * nm / s, 25.2 * nm / s);
        assert_eq!(20. * km / h + 30. * km / h, 50.0 * km / h);
    }

    #[test]
    fn speed_sub() {
        assert_eq!(55.6 * mm / d - 33.0 * mm / d, 22.6 * mm / d);
        assert_eq!(10.0 * km / ms - 5.5 * km / ms, 4.5 * km / ms);
    }

    #[test]
    fn speed_mul() {
        assert_eq!((5.1 * In / s) * 2.0, 10.2 * In / s);
        assert_eq!(3.0 * (10.5 * mi / us), 31.5 * mi / us);
        // Length * Frequency => Speed
        assert_eq!((15.0 * m) * (3.0 / ds), 45.0 * m / ds);
        // Frequency * Length => Speed
        assert_eq!((5.0 / s) * (3.0 * yd), 15.0 * yd / s);
    }

    #[test]
    fn speed_div() {
        // Length / [time unit] => Speed
        assert_eq!(10.0 * mi / h, Speed::<mi, h>::new(10.0));
        // Length / Period => Speed
        assert_eq!((45.5 * km) / (1.0 * h), Speed::<km, h>::new(45.5));
    }
}
