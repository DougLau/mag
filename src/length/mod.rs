// length.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
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
//! // let e = 1 * m * mi; // ERROR: units must match!
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
extern crate alloc;

pub(crate) mod lenpriv;

use crate::length::lenpriv::{Area, Length, Volume};
use core::ops::Mul;

/// Unit definition for Length
pub trait Unit {
    /// Unit abbreviation
    const ABBREVIATION: &'static str;

    /// Multiplication factor to convert to meters
    const M_FACTOR: f64;

    /// Multiplication factor to convert to another unit
    fn factor<T: Unit>() -> f64 {
        Self::M_FACTOR / T::M_FACTOR
    }
}

macro_rules! length_unit {
    ($(#[$meta:meta])* $unit:ident, $abbreviation:expr, $m_factor:expr) => {

        $(#[$meta])*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl Unit for $unit {
            const ABBREVIATION: &'static str = $abbreviation;
            const M_FACTOR: f64 = $m_factor;
        }

        // f64 * <unit> => Length
        impl Mul<$unit> for f64 {
            type Output = Length<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Length::new(self)
            }
        }

        // i32 * <unit> => Length
        impl Mul<$unit> for i32 {
            type Output = Length<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                Length::new(f64::from(self))
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
    "km",
    1_000.0
);

length_unit!(
    /** Meter / Metre */
    m,
    "m",
    1.0
);

length_unit!(
    /** Decimeter / Decimetre */
    dm,
    "dm",
    0.1
);

length_unit!(
    /** Centimeter / Centimetre */
    cm,
    "cm",
    0.01
);

length_unit!(
    /** Millimeter / Millimetre */
    mm,
    "mm",
    0.001
);

length_unit!(
    /** Micrometer / Micrometre */
    um,
    "μm",
    0.000_001
);

length_unit!(
    /** Nanometer / Nanometre */
    nm,
    "nm",
    0.000_000_001
);

length_unit!(
    /** Mile */
    mi,
    "mi",
    1_609.344
);

length_unit!(
    /** Foot (international) */
    ft,
    "ft",
    0.304_8
);

length_unit!(
    /** Inch (capitalized to avoid clashing with `in` keyword) */
    In,
    "in",
    0.025_4
);

length_unit!(
    /** Yard (international) */
    yd,
    "yd",
    0.914_4
);

length_unit!(
    /** League (3 mi) */
    league,
    "league",
    4_828.032
);

length_unit!(
    /** Rod (16.5 ft) */
    rod,
    "rod",
    5.029_2
);

length_unit!(
    /** Furlong (220 yd) */
    furlong,
    "furlong",
    201.168
);

length_unit!(
    /** Fathom (6 ft) */
    fathom,
    "fathom",
    1.828_8
);

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{format, string::ToString};

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
        assert_eq!((1.0 * ft).to(), (12.000000000000002 * In));
        assert_eq!((1.0 * yd).to(), (3.0 * ft));
        assert_eq!((1.0 * yd).to(), (36.0 * In));
        assert_eq!((1.0 * mi).to(), (5280.0 * ft));
        assert_eq!((1.0 * m).to(), (0.001 * km));
        assert_eq!((110.0 * cm).to(), (1.1 * m));
        assert_eq!((1.0 * cm).to(), 0.393_700_787_401_574_8 * In);
    }

    #[test]
    fn area_to() {
        assert_eq!((1.0 * ft * ft).to(), 144.00000000000006 * In * In);
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
        assert_eq!(1 * m + 1 * m, 2 * m);
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
