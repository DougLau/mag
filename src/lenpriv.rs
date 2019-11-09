// lenpriv.rs
//
// Copyright (C) 2019  Minnesota Department of Transportation
//
//! Private module for length structs
//!
use crate::length::Unit;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

/// A measurement of physical length, distance or range.
///
/// Length is a base quantity with a specific [unit].
///
/// ## Operations
///
/// * Length `+` Length `=>` Length
/// * Length `-` Length `=>` Length
/// * Length `*` f64 `=>` Length
/// * f64 `*` Length `=>` Length
/// * f64 `*` [unit] `=>` Length
/// * Length `*` Length `=>` [Area]
/// * Length `/` f64 `=>` Length
///
/// Units must be the same for operations with two Length operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::{Length, length::In};
///
/// let a = 5.5 * In;
/// let b = 4.5 * In;
/// println!("{} + {} = {}", a, b, a + b);
/// println!("{} - {} = {}", a, b, a - b);
/// ```
///
/// [Area]: struct.Area.html
/// [unit]: length/index.html
/// [to]: struct.Length.html#method.to
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Length<U> where U: Unit {
    /// Length quantity
    pub quantity: f64,
    /// Measurement unit
    unit: PhantomData<U>,
}

/// A measurement of physical area.
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
/// [unit]: length/index.html
/// [Length]: struct.Length.html
/// [Volume]: struct.Volume.html
///
/// ## Example
///
/// ```rust
/// use mag::{Area, Length, length::m};
///
/// let a = (10.0 * m) * (15.0 * m);
/// assert_eq!(a, Area::new(150.0));
/// assert_eq!(a.to_string(), "150 m²");
/// assert_eq!(a / (5.0 * m), 30.0 * m);
/// ```
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Area<U> where U: Unit {
    /// Area quantity
    pub quantity: f64,
    /// Measurement unit
    unit: PhantomData<U>,
}

/// A measurement of physical volume.
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
/// [Area]: struct.Area.html
/// [unit]: length/index.html
/// [Length]: struct.Length.html
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Volume<U> where U: Unit {
    /// Volume quantity
    pub quantity: f64,
    /// Measurement unit
    unit: PhantomData<U>,
}

impl_base_ops!(Length, Unit);
impl_base_ops!(Area, Unit);
impl_base_ops!(Volume, Unit);

impl<U> Length<U> where U: Unit {
    /// Create a new length measurement
    pub fn new(quantity: f64) -> Self {
        Length::<U> { quantity, unit: PhantomData }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Length<T> {
        let quantity = self.quantity * U::factor::<T>();
        Length::<T> { quantity, unit: PhantomData }
    }
}

impl<U> Area<U> where U: Unit {
    /// Create a new area measurement
    pub fn new(quantity: f64) -> Self {
        Area::<U> { quantity, unit: PhantomData }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Area<T> {
        let factor = U::factor::<T>() * U::factor::<T>();
        let quantity = self.quantity * factor;
        Area::<T> { quantity, unit: PhantomData }
    }
}

impl<U> Volume<U> where U: Unit {
    /// Create a new volume measurement
    pub fn new(quantity: f64) -> Self {
        Volume::<U> { quantity, unit: PhantomData }
    }

    /// Convert to specified units
    pub fn to<T: Unit>(self) -> Volume<T> {
        let factor = U::factor::<T>() * U::factor::<T>() * U::factor::<T>();
        let quantity = self.quantity * factor;
        Volume::<T> { quantity, unit: PhantomData }
    }
}

impl<U> fmt::Display for Length<U> where U: Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, U::ABBREVIATION)
    }
}

impl<U> fmt::Display for Area<U> where U: Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}²", self.quantity, U::ABBREVIATION)
    }
}

impl<U> fmt::Display for Volume<U> where U: Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}³", self.quantity, U::ABBREVIATION)
    }
}

impl<U> Mul for Length<U> where U: Unit {
    type Output = Area<U>;

    fn mul(self, other: Self) -> Self::Output {
        let quantity = self.quantity * other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}

impl<U> Mul<Length<U>> for Area<U> where U: Unit {
    type Output = Volume<U>;

    fn mul(self, other: Length<U>) -> Self::Output {
        let quantity = self.quantity * other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}

impl<U> Div<Length<U>> for Area<U> where U: Unit {
    type Output = Length<U>;

    fn div(self, other: Length<U>) -> Self::Output {
        let quantity = self.quantity / other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}

impl<U> Div<Length<U>> for Volume<U> where U: Unit {
    type Output = Area<U>;

    fn div(self, other: Length<U>) -> Self::Output {
        let quantity = self.quantity / other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}

impl<U> Div<Area<U>> for Volume<U> where U: Unit {
    type Output = Length<U>;

    fn div(self, other: Area<U>) -> Self::Output {
        let quantity = self.quantity / other.quantity;
        Self::Output { quantity, unit: PhantomData }
    }
}
