// temp.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
// Copyright (C) 2019-2021  Douglas P Lau
//
//! Units of thermodynamic temperature.
//!
//! Each unit is defined relative to degrees Kelvin with a conversion factor and
//! zero point.  They can be used to conveniently create [Temperature] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::temp::{DegC, DegF};
//!
//! let a = 22.8 * DegC;
//! let b = 98.6 * DegF;
//!
//! assert_eq!(a.to_string(), "22.8 °C");
//! assert_eq!(b.to_string(), "98.6 °F");
//! assert_eq!(b.to(), 37 * DegC);
//! ```
//!
//! # Example: Delisle
//! ```rust
//! use approx::assert_relative_eq;
//! use mag::{declare_unit, temp::DegC, measure::Temperature};
//!
//! declare_unit!(Delisle, "°D", Temperature, -2.0 / 3.0, 559.73,);
//!
//! let boiling = 0 * Delisle;
//! assert_eq!(boiling.to_string(), "0 °D");
//! assert_relative_eq!(
//!     boiling.to::<DegC>().value,
//!     100.0,
//!     max_relative = 0.000_1
//! );
//! let freezing = 0 * DegC;
//! assert_relative_eq!(
//!     freezing.to::<Delisle>().value,
//!     150.0,
//!     max_relative = 0.000_1
//! );
//! ```
use crate::declare_unit;
use crate::measure::Temperature;

declare_unit!(
    /** Degrees Celsius / Centigrade */
    DegC,
    "°C",
    Temperature,
    1.0,
    -273.15,
);

declare_unit!(
    /** Degrees Kelvin */
    DegK,
    "°K",
    Temperature,
    1.0,
    0.0,
);

declare_unit!(
    /** Degrees Fahrenheit */
    DegF,
    "°F",
    Temperature,
    5.0 / 9.0,
    -459.67,
);

declare_unit!(
    /** Degrees Rankine */
    DegR,
    "°R",
    Temperature,
    5.0 / 9.0,
    0.0,
);

declare_unit!(
    /** Degrees Réaumur */
    DegRe,
    "°Ré",
    Temperature,
    0.8,
    -273.15,
);

#[cfg(test)]
mod test {
    extern crate alloc;

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
