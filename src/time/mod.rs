// time.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
//
//! Base units of time.
//!
//! Each unit is defined relative to seconds with a conversion factor.  They
//! can be used to conveniently create [Period] and [Frequency] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::time::{s, ms, ns};
//!
//! let a = 22.8 * s; // Period<s>
//! let b = 50.6 * ms; // Period<ms>
//! let c = 60.0 / s; // Frequency<s>
//! let d = 3.1234 / ns; // Frequency<ns>
//!
//! assert_eq!(a.to_string(), "22.8 s");
//! assert_eq!(b.to_string(), "50.6 ms");
//! assert_eq!(c.to_string(), "60 ㎐");
//! assert_eq!(format!("{:.2}", d), "3.12 ㎓");
//! ```
//! [Frequency]: ../struct.Frequency.html
//! [Period]: ../struct.Period.html
//!
extern crate alloc;

pub(crate) mod timepriv;

use crate::{length, Frequency, Length, Period, Speed};
use core::ops::{Div, Mul};

/// Unit definition for time
pub trait Unit {
    /// Unit abbreviation
    const ABBREVIATION: &'static str;

    /// Inverse unit abbreviation
    const INVERSE: &'static str;

    /// Multiplication factor to convert to seconds
    const S_FACTOR: f64;

    /// Multiplication factor to convert to another unit
    fn factor<T: Unit>() -> f64 {
        Self::S_FACTOR / T::S_FACTOR
    }
}

macro_rules! time_unit {
    (
        $(#[$meta:meta])* $unit:ident,
        $abbreviation:expr,
        $inverse:expr,
        $s_factor:expr
    ) => {
        $(#[$meta])*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl Unit for $unit {
            const ABBREVIATION: &'static str = $abbreviation;
            const INVERSE: &'static str = $inverse;
            const S_FACTOR: f64 = $s_factor;
        }

        // f64 * <unit> => Period
        impl Mul<$unit> for f64 {
            type Output = Period<$unit>;
            fn mul(self, _other: $unit) -> Self::Output {
                Period::new(self)
            }
        }

        // i32 * <unit> => Period
        impl Mul<$unit> for i32 {
            type Output = Period<$unit>;
            fn mul(self, _other: $unit) -> Self::Output {
                Period::new(f64::from(self))
            }
        }

        // f64 / <unit> => Frequency
        impl Div<$unit> for f64 {
            type Output = Frequency<$unit>;
            fn div(self, _other: $unit) -> Self::Output {
                Frequency::new(self)
            }
        }

        // i32 / <unit> => Frequency
        impl Div<$unit> for i32 {
            type Output = Frequency<$unit>;
            fn div(self, _other: $unit) -> Self::Output {
                Frequency::new(f64::from(self))
            }
        }

        // Length / <unit> => Speed
        impl<L> Div<$unit> for Length<L> where L: length::Unit {
            type Output = Speed<L, $unit>;
            fn div(self, _unit: $unit) -> Self::Output {
                Speed::new(self.quantity)
            }
        }
    };
}

time_unit!(
    /** Gigasecond */
    Gs,
    "Gs",
    "nHz",
    1_000_000_000.0
);

time_unit!(
    /** Megasecond */
    Ms,
    "Ms",
    "μHz",
    1_000_000.0
);

time_unit!(
    /** Kilosecond */
    Ks,
    "Ks",
    "mHz",
    1_000.0
);

time_unit!(
    /** Week */
    wk,
    "wk",
    "/wk",
    7.0 * 24.0 * 60.0 * 60.0
);

time_unit!(
    /** Day */
    d,
    "d",
    "/d",
    24.0 * 60.0 * 60.0
);

time_unit!(
    /** Hour */
    h,
    "h",
    "/h",
    60.0 * 60.0
);

time_unit!(
    /** Minute */
    min,
    "min",
    "/min",
    60.0
);

time_unit!(
    /** Second */
    s,
    "s",
    "㎐",
    1.0
);

time_unit!(
    /** Decisecond */
    ds,
    "ds",
    "daHz",
    0.1
);

time_unit!(
    /** Millisecond */
    ms,
    "ms",
    "㎑",
    0.001
);

time_unit!(
    /** Microsecond */
    us,
    "μs",
    "㎒",
    0.000_001
);

time_unit!(
    /** Nanosecond */
    ns,
    "ns",
    "㎓",
    0.000_000_001
);

time_unit!(
    /** Picosecond */
    ps,
    "ps",
    "㎔",
    0.000_000_000_001
);

time_unit!(
    /** 14 Days */
    Fortnight,
    "fortnight",
    "/fortnight",
    14.0 * 24.0 * 60.0 * 60.0
);

#[cfg(test)]
mod test {
    extern crate alloc;

    use super::super::Frequency;
    use super::*;
    use alloc::{format, string::ToString};

    #[test]
    fn time_display() {
        assert_eq!((23.7 * s).to_string(), "23.7 s");
        assert_eq!((3.25 * h).to_string(), "3.25 h");
        assert_eq!((50.0 / s).to_string(), "50 ㎐");
        assert_eq!((2.0 / d).to_string(), "2 /d");
        assert_eq!(format!("{:.1}", 333.3333 / us), "333.3 ㎒");
    }

    #[test]
    fn time_to() {
        assert_eq!((4.75 * h).to(), 285.0 * min);
        assert_eq!((2.5 * s).to(), 2_500.0 * ms);
        assert_eq!((1_000.0 / s).to(), 1.0 / ms);
        assert_eq!((300.0 / ms).to(), 0.3 / us);
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
        assert_eq!(1.0 / (1.0 * s), 1.0 / s);
        assert_eq!(2.0 / (1.0 / min), 2.0 * min);
    }
}
