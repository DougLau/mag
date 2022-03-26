// lenpriv.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
// Copyright (C) 2019-2022  Douglas P Lau
//
//! Private module for length structs
//!
use crate::length::Unit;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

/// One dimensional _length_, _distance_ or _range_.
///
/// Length is a base quantity with a specific [unit].
///
/// ## Operations
///
/// * f64 `*` [unit] `=>` Length
/// * i32 `*` [unit] `=>` Length
/// * Length `+` Length `=>` Length
/// * Length `-` Length `=>` Length
/// * Length `*` f64 `=>` Length
/// * f64 `*` Length `=>` Length
/// * Length `*` Length `=>` [Area]
/// * Length `*` [unit] `=>` [Area]
/// * Length `/` f64 `=>` Length
///
/// Units must be the same for operations with two Length operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::length::{cm, m};
///
/// let a = 5.5 * cm;
/// let b = 4.5 * cm;
///
/// assert_eq!(a.to_string(), "5.5 cm");
/// assert_eq!((a + b).to(), 0.1 * m);
/// ```
/// [Area]: struct.Area.html
/// [unit]: length/index.html
/// [to]: struct.Length.html#method.to
///
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Length<U>
where
    U: Unit,
{
    /// Length quantity
    pub quantity: f64,

    /// Measurement unit
    unit: PhantomData<U>,
}

/// Two dimensional _area_.
///
/// Area is a derived quantity with a specific [unit] squared.
///
/// ## Operations
///
/// * Area `+` Area `=>` Area
/// * Area `-` Area `=>` Area
/// * Area `*` f64 `=>` Area
/// * Area `*` [Length] `=>` [Volume]
/// * Area `/` f64 `=>` Area
/// * Area `/` [Length] `=>` [Length]
///
/// ## Example
///
/// ```rust
/// use mag::length::m;
///
/// let a = 150.0 * m * m; // Area<m>
/// let b = 10.0 * m * 15.0 * m; // Area<m>
///
/// assert_eq!(a, b);
/// assert_eq!(a.to_string(), "150 m²");
/// assert_eq!(a / (5.0 * m), 30.0 * m);
/// ```
/// [unit]: length/index.html
/// [Length]: struct.Length.html
/// [Volume]: struct.Volume.html
///
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Area<U>
where
    U: Unit,
{
    /// Area quantity
    pub quantity: f64,

    /// Measurement unit
    unit: PhantomData<U>,
}

/// Three dimensional _volume_.
///
/// Volume is a derived quantity with a specific [unit] cubed.
///
/// ## Operations
///
/// * Volume `+` Volum `=>` Volume
/// * Volume `-` Volume `=>` Volume
/// * Volume `*` f64 `=>` Volume
/// * Volume `/` f64 `=>` Volume
/// * Volume `/` [Length] `=>` [Area]
/// * Volume `/` [Area] `=>` [Length]
///
/// ## Example
///
/// ```rust
/// use mag::length::yd;
///
/// let a = 2.5 * yd * yd * yd; // Volume<yd>
/// let b = a / (2.0 * yd); // Area<yd>
///
/// assert_eq!(a.to_string(), "2.5 yd³");
/// assert_eq!(b.to_string(), "1.25 yd²");
/// ```
/// [Area]: struct.Area.html
/// [unit]: length/index.html
/// [Length]: struct.Length.html
///
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Volume<U>
where
    U: Unit,
{
    /// Volume quantity
    pub quantity: f64,

    /// Measurement unit
    unit: PhantomData<U>,
}

impl_base_ops!(Length, Unit);
impl_base_ops!(Area, Unit);
impl_base_ops!(Volume, Unit);

impl<U> Length<U>
where
    U: Unit,
{
    /// Create a new length quantity
    pub fn new(quantity: f64) -> Self {
        Length::<U> {
            quantity,
            unit: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Length<T> {
        let quantity = self.quantity * U::factor::<T>();
        Length::new(quantity)
    }
}

impl<U> Area<U>
where
    U: Unit,
{
    /// Create a new area quantity
    pub fn new(quantity: f64) -> Self {
        Area::<U> {
            quantity,
            unit: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Area<T> {
        let factor = U::factor::<T>() * U::factor::<T>();
        let quantity = self.quantity * factor;
        Area::new(quantity)
    }
}

impl<U> Volume<U>
where
    U: Unit,
{
    /// Create a new volume quantity
    pub fn new(quantity: f64) -> Self {
        Volume::<U> {
            quantity,
            unit: PhantomData,
        }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Volume<T> {
        let factor = U::factor::<T>() * U::factor::<T>() * U::factor::<T>();
        let quantity = self.quantity * factor;
        Volume::new(quantity)
    }
}

impl<U> fmt::Display for Length<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}", U::LABEL)
    }
}

impl<U> fmt::Display for Area<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}²", U::LABEL)
    }
}

impl<U> fmt::Display for Volume<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.quantity.fmt(f)?;
        write!(f, " {}³", U::LABEL)
    }
}

// Length * Length => Area
impl<U> Mul for Length<U>
where
    U: Unit,
{
    type Output = Area<U>;
    fn mul(self, other: Self) -> Self::Output {
        Area::new(self.quantity * other.quantity)
    }
}

// Area * Length => Volume
impl<U> Mul<Length<U>> for Area<U>
where
    U: Unit,
{
    type Output = Volume<U>;
    fn mul(self, other: Length<U>) -> Self::Output {
        Volume::new(self.quantity * other.quantity)
    }
}

// Area / Length => Length
impl<U> Div<Length<U>> for Area<U>
where
    U: Unit,
{
    type Output = Length<U>;
    fn div(self, other: Length<U>) -> Self::Output {
        Length::new(self.quantity / other.quantity)
    }
}

// Volume / Length => Area
impl<U> Div<Length<U>> for Volume<U>
where
    U: Unit,
{
    type Output = Area<U>;
    fn div(self, other: Length<U>) -> Self::Output {
        Area::new(self.quantity / other.quantity)
    }
}

// Volume / Area => Length
impl<U> Div<Area<U>> for Volume<U>
where
    U: Unit,
{
    type Output = Length<U>;
    fn div(self, other: Area<U>) -> Self::Output {
        Length::new(self.quantity / other.quantity)
    }
}
