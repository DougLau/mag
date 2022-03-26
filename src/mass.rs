// mass.rs
//
// Copyright (C) 2021  Minnesota Department of Transportation
// Copyright (C) 2021  Douglas P Lau
//
//! Units of physical mass.
//!
//! Each unit is defined relative to grams with a conversion factor.  They can
//! be used to conveniently create Mass quantities.
//!
//! ## Example
//!
//! ```rust
//! use mag::mass::{g, kg};
//!
//! let a = 1.2 * kg;
//! let b = 5 * g;
//!
//! assert_eq!(a.to_string(), "1.2 kg");
//! assert_eq!(b.to_string(), "5 g");
//! ```
use crate::declare_unit;
use crate::quan::Mass;

declare_unit!(
    /** Metric Ton / Tonne */
    t,
    "t",
    Mass,
    1_000_000.0,
);

declare_unit!(
    /** Kilogram */
    kg,
    "kg",
    Mass,
    1_000.0,
);

declare_unit!(
    /** Gram */
    g,
    "g",
    Mass,
    1.0,
);

declare_unit!(
    /** Decigram */
    dg,
    "dg",
    Mass,
    0.1,
);

declare_unit!(
    /** Centigram */
    cg,
    "cg",
    Mass,
    0.01,
);

declare_unit!(
    /** Milligram */
    mg,
    "mg",
    Mass,
    0.001,
);

declare_unit!(
    /** Microgram */
    ug,
    "μg",
    Mass,
    0.000_001,
);

declare_unit!(
    /** Nanogram */
    ng,
    "ng",
    Mass,
    0.000_000_001,
);

declare_unit!(
    /** Pound (imperial) */
    lb,
    "lb",
    Mass,
    453.592_37,
);

declare_unit!(
    /** Slug (imperial) */
    sl,
    "sl",
    Mass,
    14_593.903,
);

declare_unit!(
    /** Dalton (unified atomic mass) */
    Da,
    "Da",
    Mass,
    1.660_539_066_60e-24,
);

#[cfg(test)]
mod test {
    extern crate alloc;

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
