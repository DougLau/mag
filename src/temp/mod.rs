// temp.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
//
//! Base units of temperature.
//!
//! Each unit is defined relative to degrees Kelvin with a conversion factor and
//! offset.  They can be used to conveniently create [Temperature] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::temp::{DegC, DegF};
//!
//! let a = 22.8 * DegC; // Temperature<DegC>
//! let b = 98.6 * DegF; // Temperature<DegF>
//!
//! assert_eq!(a.to_string(), "22.8 °C");
//! assert_eq!(b.to_string(), "98.6 °F");
//! assert_eq!(b.to(), 37 * DegC);
//! ```
//! [Temperature]: ../struct.Temperature.html
//!
extern crate alloc;

pub(crate) mod temppriv;

use crate::temp::temppriv::Temperature;
use core::ops::Mul;

/// Unit definition for temperature
pub trait Unit {
    /// Unit abbreviation
    const ABBREVIATION: &'static str;

    /// Multiplication factor to convert to Kelvin
    fn k_factor() -> f64;

    /// Offset to convert to Kelvin
    fn k_offset() -> f64;
}

macro_rules! temp_unit {
    (
        $(#[$meta:meta])* $unit:ident,
        $k_factor:expr,
        $k_offset:expr,
        $abbreviation:expr
    ) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl Unit for $unit {
            fn k_factor() -> f64 { $k_factor }
            fn k_offset() -> f64 { $k_offset }
            const ABBREVIATION: &'static str = { $abbreviation };
        }

        // f64 * <unit> => Temperature
        impl Mul<$unit> for f64 {
            type Output = Temperature<$unit>;
            fn mul(self, _other: $unit) -> Self::Output {
                Temperature::new(self)
            }
        }

        // i32 * <unit> => Temperature
        impl Mul<$unit> for i32 {
            type Output = Temperature<$unit>;
            fn mul(self, _other: $unit) -> Self::Output {
                Temperature::new(f64::from(self))
            }
        }
    };
}

temp_unit!(
    /** Degrees Celsius / Centigrade */
    DegC,
    1.0,
    273.15,
    "°C"
);

temp_unit!(
    /** Degrees Kelvin*/
    DegK,
    1.0,
    0.0,
    "°K"
);

temp_unit!(
    /** Degrees Fahrenheit */
    DegF,
    5.0 / 9.0,
    459.67,
    "°F"
);

temp_unit!(
    /** Degrees Rankine */
    DegR,
    5.0 / 9.0,
    0.0,
    "°R"
);

temp_unit!(
    /** Degrees Réaumur */
    DegRe,
    0.8,
    273.15,
    "°Ré"
);

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{format, string::ToString};

    #[test]
    fn temp_display() {
        assert_eq!((22.4 * DegC).to_string(), "22.4 °C");
        assert_eq!((-5.2 * DegF).to_string(), "-5.2 °F");
        assert_eq!(format!("{:.1}", 111.1111 * DegK), "111.1 °K");
        assert_eq!(format!("{:.2}", (32.0 * DegF).to::<DegC>()), "0.00 °C");
    }

    #[test]
    fn temp_to() {
        assert_eq!((32.0 * DegF).to(), 0.00000000000005684341886080802 * DegC);
        assert_eq!((0.0 * DegC).to(), 31.999999999999943 * DegF);
        assert_eq!((212.0 * DegF).to(), 100.00000000000006 * DegC);
        assert_eq!((100.0 * DegC).to(), 211.99999999999994 * DegF);
        assert_eq!((-273.15 * DegC).to(), 0.0 * DegK);
        assert_eq!((0.0 * DegK).to(), -273.15 * DegC);
    }

    #[test]
    fn temp_add() {
        assert_eq!(10.0 * DegF + 5.5 * DegF, 15.5 * DegF);
        assert_eq!(20.0 * DegC + 6.2 * DegC, 26.2 * DegC);
    }

    #[test]
    fn temp_sub() {
        assert_eq!(70.0 * DegF - 15.6 * DegF, 54.4 * DegF);
        assert_eq!(40.0 * DegC - 16.1 * DegC, 23.9 * DegC);
    }
}
