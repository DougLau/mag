// measure.rs
//
// Copyright (C) 2021  Douglas P Lau
//

use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

pub struct Length;
pub struct Time;

/// Measure of mass.
///
/// Mass is a "base quantity", with units such as `kg` and `lb`.
///
/// Units must be the same for operations with two Mass operands.  The [to]
/// method can be used for conversion.
///
/// ## Example
///
/// ```rust
/// use mag::mass::{kg, lb};
///
/// let a = 2.5 * kg;
/// assert_eq!(a.to_string(), "2.5 kg");
/// assert_eq!(a + 4.5 * kg, 7 * kg);
/// assert_eq!(a.to(), 5.511556554621939 * lb);
/// ```
///
/// # Example: Solar Mass Units
/// ```rust
/// use mag::{declare_unit, mass::kg, measure::Mass};
///
/// declare_unit!(M, "M☉", Mass, 1.988_47e33,);
///
/// let sun = 1 * M;
/// assert_eq!(sun.to_string(), "1 M☉");
/// assert_eq!(sun.to(), 1.988_47e30 * kg);
/// ```
///
/// [mass]: struct.Mass.html
/// [unit]: ../mass/index.html
/// [to]: struct.Quantity.html#method.to
///
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Mass;

/// Thermodynamic _temperature_.
///
/// Temperature is a "base quantity" with units such as DegC and DegF.
///
/// ## Example
///
/// ```rust
/// use mag::temp::{DegC, DegF};
///
/// let a = 98.6 * DegF;
/// assert_eq!(a.to_string(), "98.6 °F");
/// assert_eq!(a.to(), 37 * DegC);
/// assert_eq!((22.8 * DegC).to_string(), "22.8 °C");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Temperature;

/// Unit of measure
pub trait Unit {
    /// Unit abbreviation
    const ABBREVIATION: &'static str;

    /// Factor to convert to base unit
    const FACTOR: f64;

    /// Value of (absolute) zero
    const ZERO: f64;

    /// Measure (length, mass, etc.)
    type Measure;

    /// Convert a value to another unit of the same measure
    fn convert<T>(val: f64) -> f64
    where
        T: Unit<Measure = Self::Measure>,
    {
        val * (Self::FACTOR / T::FACTOR)
    }
}

/// Marker trait for units which can be scaled by multiplication (or division)
pub trait MulUnit {}

impl MulUnit for Mass {}

/// Define a custom [unit] of measure.
///
/// * `unit` Unit struct name
/// * `abbreviation` Standard unit abbreviation
/// * `measure` A base or derived measure
/// * `factor` Factor to convert
/// * `zero` (Absolute) zero point
///
/// [Unit]: measure/trait.Unit.html
#[macro_export]
macro_rules! declare_unit {
    ($(#[$doc:meta])*
        $unit:ident,
        $abbreviation:expr,
        $measure:ident,
        $factor:expr,
    ) => {
        $(#[$doc])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $unit;

        impl $crate::measure::Unit for $unit {
            type Measure = $measure;
            const ABBREVIATION: &'static str = $abbreviation;
            const FACTOR: f64 = $factor;
            const ZERO: f64 = 0.0;
        }

        impl core::ops::Mul<$unit> for f64 {
            type Output = $crate::measure::Quantity<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Self::Output::new(self)
            }
        }

        impl core::ops::Mul<$unit> for i32 {
            type Output = $crate::measure::Quantity<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
    ($(#[$doc:meta])*
        $unit:ident,
        $abbreviation:expr,
        $measure:ident,
        $factor:expr,
        $zero:expr,
    ) => {
        $(#[$doc])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $unit;

        impl $crate::measure::Unit for $unit {
            type Measure = $measure;
            const ABBREVIATION: &'static str = $abbreviation;
            const FACTOR: f64 = $factor;
            const ZERO: f64 = $zero;

            /// Convert a value to another unit of the same measure
            fn convert<T>(val: f64) -> f64
            where
                T: $crate::measure::Unit<Measure = Self::Measure>,
            {
                let v = (val - Self::ZERO) * Self::FACTOR;
                v / T::FACTOR + T::ZERO
            }
        }

        impl core::ops::Mul<$unit> for f64 {
            type Output = $crate::measure::Quantity<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Self::Output::new(self)
            }
        }

        impl core::ops::Mul<$unit> for i32 {
            type Output = $crate::measure::Quantity<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
}

/// Quantity is a value with an associated unit
///
/// ## Operations
///
/// * f64 `*` [Unit] `=>` Quantity<Unit>
/// * i32 `*` [Unit] `=>` Quantity<Unit>
/// * Mass `+` Mass `=>` Mass
/// * Mass `-` Mass `=>` Mass
/// * Mass `*` f64 `=>` Mass
/// * f64 `*` Mass `=>` Mass
/// * Mass `/` f64 `=>` Mass
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<U>
where
    U: Unit,
{
    /// Quantity of units
    pub value: f64,

    /// Unit of measure
    unit: PhantomData<U>,
}

impl<U> Quantity<U>
where
    U: Unit,
{
    /// Create a new measure
    pub fn new<V>(value: V) -> Self
    where
        V: Into<f64>,
    {
        Self {
            value: value.into(),
            unit: PhantomData,
        }
    }

    /// Convert quantity to the specified units
    pub fn to<T>(self) -> Quantity<T>
    where
        T: Unit<Measure = <U>::Measure>,
    {
        Quantity::new(U::convert::<T>(self.value))
    }
}

impl<U> fmt::Display for Quantity<U>
where
    U: Unit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<U> Add for Quantity<U>
where
    U: Unit,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.value + other.value)
    }
}

impl<U> Sub for Quantity<U>
where
    U: Unit,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.value - other.value)
    }
}

impl<U, V> Mul<V> for Quantity<U>
where
    U: Unit,
    V: Into<f64>,
{
    type Output = Self;
    fn mul(self, scalar: V) -> Self::Output {
        Self::new(self.value * scalar.into())
    }
}

impl<U, M> Mul<Quantity<U>> for f64
where
    U: Unit<Measure = M>,
    M: MulUnit,
{
    type Output = Quantity<U>;
    fn mul(self, quan: Self::Output) -> Self::Output {
        Self::Output::new(self * quan.value)
    }
}

impl<U, M> Div<f64> for Quantity<U>
where
    U: Unit<Measure = M>,
    M: MulUnit,
{
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}
