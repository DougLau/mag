// length.rs
//
// Copyright (C) 2019-2020  Minnesota Department of Transportation
//
//! Base units of length.
//!
//! Each unit is defined relative to meters with a conversion factor.  They can
//! be used to conveniently create [Length], [Area] and [Volume] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::length::{cm, m, mi, yd};
//!
//! let a = 25.5 * cm; // Length<cm>
//! let b = 5.6 * mi; // Length<mi>
//! let c = 1.2 * m * m; // Area<m>
//! let d = 5.259 * yd * yd * yd; // Volume<yd>
//! // let e = 1.0 * m * mi; // ERROR: units must match!
//!
//! assert_eq!(a.to_string(), "25.5 cm");
//! assert_eq!(b.to_string(), "5.6 mi");
//! assert_eq!(c.to_string(), "1.2 m²");
//! assert_eq!(format!("{:.2}", d), "5.26 yd³");
//! ```
//! [Area]: ../struct.Area.html
//! [Length]: ../struct.Length.html
//! [Volume]: ../struct.Volume.html
//!
use crate::lenpriv::{Area, Length, Volume};
use std::ops::Mul;

/// Unit definition for Length
pub trait Unit {
    /// Multiplication factor to convert to meters
    fn m_factor() -> f64;
    /// Multiplication factor to convert to another unit
    fn factor<T: Unit>() -> f64 {
        // Use 14 digits precision for conversion constants.
        // The significand of f64 is 52 bits, which is about 15 digits.
        const PRECISION: f64 = 100_000_000_000_000.0;
        // This gets compiled down to a constant value
        ((Self::m_factor() / T::m_factor()) * PRECISION).round() / PRECISION
    }
    /// Unit abbreviation
    const ABBREVIATION: &'static str;
}

macro_rules! length_unit {
    ($(#[$meta:meta])* $unit:ident, $m_factor:expr, $abbreviation:expr) => {

        $(#[$meta])*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl Unit for $unit {
            fn m_factor() -> f64 { $m_factor }
            const ABBREVIATION: &'static str = { $abbreviation };
        }

        // f64 * <unit> => Length
        impl Mul<$unit> for f64 {
            type Output = Length<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Length::new(self)
            }
        }

        // Length * <unit> => Area
        impl Mul<$unit> for Length<$unit> {
            type Output = Area<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Area::new(self.quantity)
            }
        }

        // Area * <unit> => Volume
        impl Mul<$unit> for Area<$unit> {
            type Output = Volume<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Volume::new(self.quantity)
            }
        }
    };
}

length_unit!(
    /** Kilometer / Kilometre */
    km,
    1_000.0,
    "km"
);
length_unit!(
    /** Meter / Metre */
    m,
    1.0,
    "m"
);
length_unit!(
    /** Decimeter / Decimetre */
    dm,
    0.1,
    "dm"
);
length_unit!(
    /** Centimeter / Centimetre */
    cm,
    0.01,
    "cm"
);
length_unit!(
    /** Millimeter / Millimetre */
    mm,
    0.001,
    "mm"
);
length_unit!(
    /** Micrometer / Micrometre */
    um,
    0.000_001,
    "μm"
);
length_unit!(
    /** Nanometer / Nanometre */
    nm,
    0.000_000_001,
    "nm"
);
length_unit!(
    /** Mile */
    mi,
    1609.344,
    "mi"
);
length_unit!(
    /** Foot */
    ft,
    0.3048,
    "ft"
);
length_unit!(
    /** Inch */
    In,
    0.0254,
    "in"
);
length_unit!(
    /** Yard */
    yd,
    0.9144,
    "yd"
);

#[cfg(feature = "obscure-units")]
length_unit!(
    /** League */
    league,
    4828.032,
    "league"
);
#[cfg(feature = "obscure-units")]
length_unit!(
    /** Rod */
    rod,
    5.0292,
    "rod"
);
#[cfg(feature = "obscure-units")]
length_unit!(
    /** Furlong */
    furlong,
    201.168,
    "furlong"
);
#[cfg(feature = "obscure-units")]
length_unit!(
    /** Fathom */
    fathom,
    1.8288,
    "fathom"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len_display() {
        assert_eq!((2.5 * km).to_string(), "2.5 km");
        assert_eq!((10.0 * m).to_string(), "10 m");
        assert_eq!((11.1 * dm).to_string(), "11.1 dm");
        assert_eq!((25.0 * cm).to_string(), "25 cm");
        assert_eq!((101.01 * mm).to_string(), "101.01 mm");
        assert_eq!((3.9 * um).to_string(), "3.9 μm");
        assert_eq!((2.22 * mi).to_string(), "2.22 mi");
        assert_eq!((0.5 * ft).to_string(), "0.5 ft");
        assert_eq!((6. * In).to_string(), "6 in");
        assert_eq!((100.0 * yd).to_string(), "100 yd");
    }

    #[test]
    fn area_display() {
        assert_eq!((1.0 * m * m).to_string(), "1 m²");
        assert_eq!((18.5 * In * In).to_string(), "18.5 in²");
        assert_eq!(format!("{:.2}", 1.234 * cm * cm), "1.23 cm²");
    }

    #[test]
    fn volume_display() {
        assert_eq!((123.0 * um * um * um).to_string(), "123 μm³");
        assert_eq!((54.3 * In * In * In).to_string(), "54.3 in³");
    }

    #[test]
    fn len_to() {
        assert_eq!((1.0 * ft).to(), (12.0 * In));
        assert_eq!((1.0 * yd).to(), (3.0 * ft));
        assert_eq!((1.0 * yd).to(), (36.0 * In));
        assert_eq!((1.0 * mi).to(), (5280.0 * ft));
        assert_eq!((1.0 * m).to(), (0.001 * km));
        assert_eq!((110.0 * cm).to(), (1.1 * m));
        assert_eq!((1.0 * cm).to(), 0.393_700_787_401_57 * In);
    }

    #[test]
    fn area_to() {
        assert_eq!((1.0 * ft * ft).to(), 144.0 * In * In);
        assert_eq!((1.0 * m * m).to(), 10_000.0 * cm * cm);
    }

    #[test]
    fn volume_to() {
        assert_eq!((2.0 * yd * yd * yd).to(), 54.0 * ft * ft * ft);
        assert_eq!((4.8 * cm * cm * cm).to(), 4_800.0 * mm * mm * mm);
    }

    #[test]
    fn len_add() {
        assert_eq!(1.0 * m + 1.0 * m, 2.0 * m);
        assert_eq!(10.0 * ft + 2.0 * ft, 12.0 * ft);
        assert_eq!(6.0 * In + 6.0 * In, 12.0 * In);
    }

    #[test]
    fn area_add() {
        assert_eq!(12.0 * yd * yd + 15.0 * yd * yd, 27.0 * yd * yd);
        assert_eq!(25.6 * km * km + 15.4 * km * km, 41.0 * km * km);
    }

    #[test]
    fn volume_add() {
        assert_eq!(
            25.0 * mm * mm * mm + 5.1 * mm * mm * mm,
            30.1 * mm * mm * mm
        );
        assert_eq!(1.2 * In * In * In + 3.8 * In * In * In, 5.0 * In * In * In);
    }

    #[test]
    fn len_sub() {
        assert_eq!(5.0 * km - 1.0 * km, 4.0 * km);
        assert_eq!(500.0 * mm - 100.0 * mm, 400.0 * mm);
    }

    #[test]
    fn area_sub() {
        assert_eq!(5.0 * mi * mi - 2.5 * mi * mi, 2.5 * mi * mi);
    }

    #[test]
    fn volume_sub() {
        assert_eq!(10.0 * m * m * m - 4.5 * m * m * m, 5.5 * m * m * m);
    }

    #[test]
    fn len_mul() {
        assert_eq!((3.0 * m) * (3.0 * m), 9.0 * m * m);
        assert_eq!((3.0 * nm) * 3.0, 9.0 * nm);
        assert_eq!(3.0 * (3.0 * m), 9.0 * m);
        assert_eq!((10.0 * In) * (5.0 * In), 50.0 * In * In);
    }

    #[test]
    fn area_mul() {
        assert_eq!(3.0 * dm * dm * 2.5, 7.5 * dm * dm);
        assert_eq!(4.0 * (3.0 * dm * dm), 12.0 * dm * dm);
        assert_eq!(123.0 * mm * mm * (2.0 * mm), 246.0 * mm * mm * mm);
        assert_eq!(123.0 * mm * mm * 2.0 * mm, 246.0 * mm * mm * mm);
    }

    #[test]
    fn volume_mul() {
        assert_eq!(8.0 * um * um * um * 1.5, 12.0 * um * um * um);
        assert_eq!(4.0 * (2.5 * km * km * km), 10.0 * km * km * km);
    }

    #[test]
    fn len_div() {
        assert_eq!((5.0 * ft) / 5.0, 1.0 * ft);
    }

    #[test]
    fn area_div() {
        assert_eq!((500.0 * cm * cm) / 5.0, 100.0 * cm * cm);
        assert_eq!(40.0 * nm * nm / (10.0 * nm), 4.0 * nm);
    }

    #[test]
    fn volume_div() {
        assert_eq!((50.0 * mm * mm * mm) / 10.0, 5.0 * mm * mm * mm);
        assert_eq!((40.0 * yd * yd * yd) / (2.0 * yd), 20.0 * yd * yd);
        assert_eq!((25.0 * In * In * In) / (5.0 * In * In), 5.0 * In);
    }
}
