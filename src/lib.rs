//! Mag is a library for dealing with units of measure.  Magnitude!
//!
//! ## Highlights
//!
//! * Easy to understand and use
//! * No external dependencies
//! * Fast compile time
//! * Units are not discarded when creating quantities
//!
//! ## Example
//!
//! ```rust
//! use mag::{length::{ft, m, mi}, time::{h, s}};
//!
//! let a = 1.0 * ft; // Length<ft>
//! let b = a.to::<m>(); // convert to Length<m>
//! let c = 30.0 * s; // Period<s>
//! let d = 60.0 / s; // Frequency<s>
//! let e = 55.0 * mi / h; // Speed<mi, h>
//!
//! assert_eq!(a.to_string(), "1 ft");
//! assert_eq!(b.to_string(), "0.3048 m");
//! assert_eq!(c.to_string(), "30 s");
//! assert_eq!(d.to_string(), "60 ㎐");
//! assert_eq!(e.to_string(), "55 mi/h");
//! ```
//!
//! ## Room For Improvement
//!
//! * Small set of quantities and units implemented
//! * Quantities are f64 only
//!
#![forbid(unsafe_code)]

// Implement basic ops for a quantity struct
macro_rules! impl_base_ops {
    ($quan:ident, $unit:path) => {
        impl<U> Add for $quan<U> where U: $unit {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self::new(self.quantity + other.quantity)
            }
        }
        impl<U> Sub for $quan<U> where U: $unit {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self::new(self.quantity - other.quantity)
            }
        }
        impl<U> Mul<f64> for $quan<U> where U: $unit {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self::Output {
                Self::new(self.quantity * scalar)
            }
        }
        impl<U> Mul<$quan<U>> for f64 where U: $unit {
            type Output = $quan<U>;
            fn mul(self, other: $quan<U>) -> Self::Output {
                Self::Output::new(self * other.quantity)
            }
        }
        impl<U> Div<f64> for $quan<U> where U: $unit {
            type Output = Self;
            fn div(self, scalar: f64) -> Self::Output {
                Self::new(self.quantity / scalar)
            }
        }
    }
}

pub mod length;
mod lenpriv;
pub mod time;
mod timepriv;
mod speed;

pub use lenpriv::{Area, Length, Volume};
pub use timepriv::{Frequency, Period};
pub use speed::Speed;
