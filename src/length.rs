// length.rs
//
// Copyright (C) 2019  Minnesota Department of Transportation
//
//! Base units of length.
//!
//! Each unit is defined relative to meters with a conversion factor.  They can
//! be used to conveniently create [Length] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::length::cm;
//!
//! let a = 25.5 * cm;
//! assert_eq!(a.to_string(), "25.5 cm");
//! ```
//! [Length]: ../struct.Length.html
//!
use crate::lenpriv::Length;
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

        impl Mul<$unit> for f64 {
            type Output = Length<$unit>;

            fn mul(self, _other: $unit) -> Self::Output {
                let quantity = self;
                //Self::Output { quantity, unit: PhantomData }
                Length::new(quantity)
            }
        }
    };
}

length_unit!(/** Kilometer / Kilometre */ km, 1_000.0, "km");
length_unit!(/** Meter / Metre */ m, 1.0, "m");
length_unit!(/** Decimeter / Decimetre */ dm, 0.1, "dm");
length_unit!(/** Centimeter / Centimetre */ cm, 0.01, "cm");
length_unit!(/** Millimeter / Millimetre */ mm, 0.001, "mm");
length_unit!(/** Micrometer / Micrometre */ um, 0.000_001, "μm");
length_unit!(/** Nanometer / Nanometre */ nm, 0.000_000_001, "nm");
length_unit!(/** Mile */ mi, 1609.344, "mi");
length_unit!(/** Foot */ ft, 0.3048, "ft");
length_unit!(/** Inch */ In, 0.0254, "in");
length_unit!(/** Yard */ yd, 0.9144, "yd");

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;

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
        assert_eq!(Area::<m>::new(1.0).to_string(), "1 m²");
        assert_eq!(Area::<In>::new(18.5).to_string(), "18.5 in²");
    }

    #[test]
    fn volume_display() {
        assert_eq!(Volume::<um>::new(123.0).to_string(), "123 μm³");
        assert_eq!(Volume::<In>::new(54.3).to_string(), "54.3 in³");
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
        assert_eq!(Area::<ft>::new(1.0).to(), Area::<In>::new(144.0));
        assert_eq!(Area::<m>::new(1.0).to(), Area::<cm>::new(10_000.0));
    }

    #[test]
    fn volume_to() {
        assert_eq!(Volume::<yd>::new(2.0).to(), Volume::<ft>::new(54.0));
        assert_eq!(Volume::<cm>::new(4.8).to(), Volume::<mm>::new(4_800.0));
    }

    #[test]
    fn len_add() {
        assert_eq!(1.0 * m + 1.0 * m, 2.0 * m);
        assert_eq!(10.0 * ft + 2.0 * ft, 12.0 * ft);
        assert_eq!(6.0 * In + 6.0 * In, 12.0 * In);
    }

    #[test]
    fn area_add() {
        assert_eq!(
            Area::<yd>::new(12.0) + Area::<yd>::new(15.0),
            Area::<yd>::new(27.0)
        );
        assert_eq!(
            Area::<km>::new(25.6) + Area::<km>::new(15.4),
            Area::<km>::new(41.0)
        );
    }

    #[test]
    fn volume_add() {
        assert_eq!(
            Volume::<mm>::new(25.0) + Volume::<mm>::new(5.1),
            Volume::<mm>::new(30.1)
        );
        assert_eq!(
            Volume::<In>::new(1.2) + Volume::<In>::new(3.8),
            Volume::<In>::new(5.0)
        );
    }

    #[test]
    fn len_sub() {
        assert_eq!(5.0 * km - 1.0 * km, 4.0 * km);
        assert_eq!(500.0 * mm - 100.0 * mm, 400.0 * mm);
    }

    #[test]
    fn area_sub() {
        assert_eq!(
            Area::<mi>::new(5.0) - Area::<mi>::new(2.5),
            Area::<mi>::new(2.5)
        );
    }

    #[test]
    fn volume_sub() {
        assert_eq!(
            Volume::<m>::new(10.0) - Volume::<m>::new(4.5),
            Volume::<m>::new(5.5)
        );
    }

    #[test]
    fn len_mul() {
        assert_eq!((3.0 * m) * (3.0 * m), Area::<m>::new(9.0));
        assert_eq!((3.0 * nm) * 3.0, 9.0 * nm);
        assert_eq!(3.0 * (3.0 * m), 9.0 * m);
        assert_eq!((10.0 * In) * (5.0 * In), Area::<In>::new(50.0));
    }

    #[test]
    fn area_mul() {
        assert_eq!(Area::<dm>::new(3.0) * 2.5, Area::<dm>::new(7.5));
        assert_eq!(4.0 * Area::<dm>::new(3.0), Area::<dm>::new(12.0));
        assert_eq!(
            Area::<mm>::new(123.0) * Length::<mm>::new(2.0),
            Volume::<mm>::new(246.0)
        );
    }

    #[test]
    fn volume_mul() {
        assert_eq!(Volume::<um>::new(8.0) * 1.5, Volume::<um>::new(12.0));
        assert_eq!(4.0 * Volume::<km>::new(2.5), Volume::<km>::new(10.0));
    }

    #[test]
    fn len_div() {
        assert_eq!((5.0 * ft) / 5.0, 1.0 * ft);
    }

    #[test]
    fn area_div() {
        assert_eq!(Area::<cm>::new(500.0) / 5.0, Area::<cm>::new(100.0));
        assert_eq!(Area::<nm>::new(40.0) / Length::<nm>::new(10.0), 4.0 * nm);
    }

    #[test]
    fn volume_div() {
        assert_eq!(Volume::<mm>::new(50.0) / 10.0, Volume::<mm>::new(5.0));
        assert_eq!(Volume::<yd>::new(40.0) / (2.0 * yd), Area::<yd>::new(20.0));
        assert_eq!(Volume::<In>::new(25.0) / Area::<In>::new(5.0), 5.0 * In);
    }
}
