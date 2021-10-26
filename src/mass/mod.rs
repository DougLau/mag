// mass.rs
//
// Copyright (C) 2021  Minnesota Department of Transportation
//
//! Units of physical mass.
//!
//! Each unit is defined relative to grams with a conversion factor.  They can
//! be used to conveniently create [Mass] structs.
//!
//! ## Example
//!
//! ```rust
//! use mag::mass::{g, kg};
//!
//! let a = 1.2 * kg; // Mass<kg>
//! let b = 5 * g; // Mass<g>
//!
//! assert_eq!(a.to_string(), "1.2 kg");
//! assert_eq!(b.to_string(), "5 g");
//! ```
//! [Mass]: ../struct.Mass.html
//!
extern crate alloc;

pub(crate) mod masspriv;

/// Unit definition for Mass
pub trait Unit {
    /// Unit abbreviation
    const ABBREVIATION: &'static str;

    /// Multiplication factor to convert to grams
    const G_FACTOR: f64;

    /// Multiplication factor to convert to another unit
    fn factor<T: Unit>() -> f64 {
        Self::G_FACTOR / T::G_FACTOR
    }
}

/// Define a custom [unit] of [mass]
///
/// * `unit` Unit struct name
/// * `abbreviation` Standard unit abbreviation
/// * `g_factor` Factor to convert to grams
///
/// # Example: Solar Mass
/// ```rust
/// use mag::{mass_unit, mass::kg};
///
/// mass_unit!(M, "M☉", 1.988_47e33);
///
/// let sun = 1 * M;
/// assert_eq!(sun.to(), 1.988_47e30 * kg);
/// assert_eq!(sun.to_string(), "1 M☉");
/// ```
///
/// [mass]: struct.Mass.html
/// [unit]: mass/trait.Unit.html
#[macro_export]
macro_rules! mass_unit {
    ($(#[$doc:meta])* $unit:ident, $abbreviation:expr, $g_factor:expr) => {

        $(#[$doc])*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $unit;

        impl $crate::mass::Unit for $unit {
            const ABBREVIATION: &'static str = $abbreviation;
            const G_FACTOR: f64 = $g_factor;
        }

        // f64 * <unit> => Mass
        impl core::ops::Mul<$unit> for f64 {
            type Output = $crate::Mass<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                $crate::Mass::new(self)
            }
        }

        // i32 * <unit> => Mass
        impl core::ops::Mul<$unit> for i32 {
            type Output = $crate::Mass<$unit>;
            fn mul(self, _unit: $unit) -> Self::Output {
                $crate::Mass::new(f64::from(self))
            }
        }
    };
}

mass_unit!(
    /** Metric Ton / Tonne */
    t,
    "t",
    1_000_000.0
);

mass_unit!(
    /** Kilogram */
    kg,
    "kg",
    1_000.0
);

mass_unit!(
    /** Gram */
    g,
    "g",
    1.0
);

mass_unit!(
    /** Decigram */
    dg,
    "dg",
    0.1
);

mass_unit!(
    /** Centigram */
    cg,
    "cg",
    0.01
);

mass_unit!(
    /** Milligram */
    mg,
    "mg",
    0.001
);

mass_unit!(
    /** Microgram */
    ug,
    "μg",
    0.000_001
);

mass_unit!(
    /** Nanogram */
    ng,
    "ng",
    0.000_000_001
);

mass_unit!(
    /** Pound (imperial) */
    lb,
    "lb",
    453.592_37
);

mass_unit!(
    /** Slug (imperial) */
    sl,
    "sl",
    14_593.903
);

mass_unit!(
    /** Dalton (unified atomic mass) */
    Da,
    "Da",
    1.660_539_066_60e-24
);

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn mass_display() {
        assert_eq!((2.5 * kg).to_string(), "2.5 kg");
        assert_eq!((10.0 * g).to_string(), "10 g");
        assert_eq!((11.1 * dg).to_string(), "11.1 dg");
        assert_eq!((25.0 * cg).to_string(), "25 cg");
        assert_eq!((101.01 * mg).to_string(), "101.01 mg");
        assert_eq!((3.9 * ug).to_string(), "3.9 μg");
    }

    #[test]
    fn mass_to() {
        assert_eq!((1.0 * g).to(), (0.001 * kg));
        assert_eq!((110.0 * cg).to(), (1.1 * g));
    }

    #[test]
    fn mass_add() {
        assert_eq!(1.0 * g + 1.0 * g, 2.0 * g);
        assert_eq!(1 * g + 1 * g, 2 * g);
    }

    #[test]
    fn mass_sub() {
        assert_eq!(5.0 * kg - 1.0 * kg, 4.0 * kg);
        assert_eq!(500.0 * mg - 100.0 * mg, 400.0 * mg);
    }

    #[test]
    fn mass_mul() {
        assert_eq!((3.0 * ng) * 3.0, 9.0 * ng);
        assert_eq!(3.0 * (3.0 * g), 9.0 * g);
    }

    #[test]
    fn mass_div() {
        assert_eq!((5.0 * dg) / 5.0, 1.0 * dg);
    }
}
