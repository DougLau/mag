// time.rs
//
// Copyright (C) 2019  Minnesota Department of Transportation
//
//! Base units of time.
//!
//! Each unit is defined relative to seconds with a conversion factor.  They
//! can be used to conveniently create [Period] and [Frequency] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::time::{s, ms};
//!
//! let a = 22.8 * s; // Period<s>
//! let b = 50.6 * ms; // Period<ms>
//! let c = 60.0 / s; // Frequency<s>
//!
//! assert_eq!(a.to_string(), "22.8 s");
//! assert_eq!(b.to_string(), "50.6 ms");
//! assert_eq!(c.to_string(), "60 Hz");
//! ```
//!
//! [Frequency]: ../struct.Frequency.html
//! [Period]: ../struct.Period.html
//!
use super::{Frequency, Period};
use std::ops::{Div, Mul};

/// Unit definition for time
pub trait Unit {
    /// Multiplication factor to convert to seconds
    fn s_factor() -> f64;
    /// Multiplication factor to convert to another unit
    fn factor<T: Unit>() -> f64 {
        Self::s_factor() / T::s_factor()
    }
    /// Unit abbreviation
    const ABBREVIATION: &'static str;
    /// Inverse unit abbreviation
    const INVERSE: &'static str;
}

macro_rules! time_unit {
    (
        $(#[$meta:meta])* $unit:ident,
        $s_factor:expr,
        $abbreviation:expr,
        $inverse:expr
    ) => {
        $(#[$meta])*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl Unit for $unit {
            fn s_factor() -> f64 { $s_factor }
            const ABBREVIATION: &'static str = { $abbreviation };
            const INVERSE: &'static str = { $inverse };
        }

        impl Mul<$unit> for f64 {
            type Output = Period<$unit>;

            fn mul(self, _other: $unit) -> Self::Output {
                let quantity = self;
                Period::new(quantity)
            }
        }

        impl Div<$unit> for f64 {
            type Output = Frequency<$unit>;

            fn div(self, _other: $unit) -> Self::Output {
                let quantity = self;
                Frequency::new(quantity)
            }
        }
    };
}

time_unit!(/** Gigasecond */ Gs, 1_000_000_000.0, "Gs", "GHz");
time_unit!(/** Megasecond */ Ms, 1_000_000.0, "Ms", "MHz");
time_unit!(/** Kilosecond */ Ks, 1_000.0, "Ks", "KHz");
time_unit!(/** 14 Days */ Fortnight, 14.0 * 24.0 * 60.0 * 60.0, "fortnight",
    "/fortnight");
time_unit!(/** Week */ wk, 7.0 * 24.0 * 60.0 * 60.0, "wk", "/wk");
time_unit!(/** Day */ d, 24.0 * 60.0 * 60.0, "d", "/d");
time_unit!(/** Hour */ h, 60.0 * 60.0, "h", "/h");
time_unit!(/** Minute */ min, 60.0, "min", "/min");
time_unit!(/** Second */ s, 1.0, "s", "Hz");
time_unit!(/** Decisecond */ ds, 0.1, "ds", "dHz");
time_unit!(/** Millisecond */ ms, 0.001, "ms", "mHz");
time_unit!(/** Microsecond */ us, 0.000_001, "μs", "μHz");
time_unit!(/** Nanosecond */ ns, 0.000_000_001, "ns", "nHz");

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Frequency;

    #[test]
    fn time_display() {
        assert_eq!((23.7 * s).to_string(), "23.7 s");
        assert_eq!((3.25 * h).to_string(), "3.25 h");
        assert_eq!((50.0 / s).to_string(), "50 Hz");
        assert_eq!((2.0 / d).to_string(), "2 /d");
    }

    #[test]
    fn time_to() {
        assert_eq!((4.75 * h).to(), 285.0 * min);
        assert_eq!((2.5 * s).to(), 2_500.0 * ms);
        assert_eq!((1_000.0 / s).to(), Frequency::<Ks>::new(1.0));
    }

    #[test]
    fn time_add() {
        assert_eq!(3.5 * d + 1.25 * d, 4.75 * d);
        assert_eq!(1.0 * wk + 2.1 * wk, 3.1 * wk);
        assert_eq!(5.0 / ns + 4.0 / ns, 9.0 / ns);
    }

    #[test]
    fn time_sub() {
        assert_eq!(567.8 * us - 123.4 * us, 444.4 * us);
        assert_eq!(23.0 / ms - 12.0 / ms, 11.0 / ms);
    }

    #[test]
    fn time_mul() {
        assert_eq!((6.5 * ns) * 12.0, 78.0 * ns);
        assert_eq!(4.0 * (1.5 * h), 6.0 * h);
        assert_eq!(2.5 / ds * 2.0, 5.0 / ds);
    }

    #[test]
    fn time_div() {
        assert_eq!(5. / h, Frequency::<h>::new(5.0));
        assert_eq!(60.0 / s, Frequency::<s>::new(60.0));
        assert_eq!(1.0 / (1.0 * s), Frequency::<s>::new(1.0));
        assert_eq!(2.0 / (1.0 / min), Period::<min>::new(2.0));
    }
}
