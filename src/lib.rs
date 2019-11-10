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
//! use mag::length::{ft, m};
//!
//! let a = 1.0 * ft; // Length<ft>
//! let b = a.to::<m>(); // Length<m>
//!
//! assert_eq!(b.to_string(), "0.3048 m");
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
                let quantity = self.quantity + other.quantity;
                Self { quantity, unit: PhantomData }
            }
        }
        impl<U> Sub for $quan<U> where U: $unit {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                let quantity = self.quantity - other.quantity;
                Self { quantity, unit: PhantomData }
            }
        }
        impl<U> Mul<f64> for $quan<U> where U: $unit {
            type Output = Self;
            fn mul(self, other: f64) -> Self::Output {
                let quantity = self.quantity * other;
                Self::Output { quantity, unit: PhantomData }
            }
        }
        impl<U> Mul<$quan<U>> for f64 where U: $unit {
            type Output = $quan<U>;
            fn mul(self, other: $quan<U>) -> Self::Output {
                let quantity = self * other.quantity;
                Self::Output { quantity, unit: PhantomData }
            }
        }
        impl<U> Div<f64> for $quan<U> where U: $unit {
            type Output = Self;
            fn div(self, other: f64) -> Self::Output {
                let quantity = self.quantity / other;
                Self::Output { quantity, unit: PhantomData }
            }
        }
    }
}

pub mod length;
mod lenpriv;
pub mod time;
mod timepriv;

pub use lenpriv::{Area, Length, Volume};
pub use timepriv::{Frequency, Period};
