// lib.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
// Copyright (C) 2019-2021  Douglas P Lau
//
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]

// Implement basic ops for a quantity struct
macro_rules! impl_base_ops {
    ($quan:ident, $unit:path) => {
        // <quan> + <quan> => <quan>
        impl<U> Add for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self::new(self.quantity + other.quantity)
            }
        }

        // <quan> - <quan> => <quan>
        impl<U> Sub for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self::new(self.quantity - other.quantity)
            }
        }

        // <quan> * f64 => <quan>
        impl<U> Mul<f64> for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self::Output {
                Self::new(self.quantity * scalar)
            }
        }

        // <quan> * i32 => <quan>
        impl<U> Mul<i32> for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn mul(self, scalar: i32) -> Self::Output {
                Self::new(self.quantity * f64::from(scalar))
            }
        }

        // f64 * <quan> => <quan>
        impl<U> Mul<$quan<U>> for f64
        where
            U: $unit,
        {
            type Output = $quan<U>;
            fn mul(self, other: $quan<U>) -> Self::Output {
                Self::Output::new(self * other.quantity)
            }
        }

        // <quan> / f64 => <quan>
        impl<U> Div<f64> for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn div(self, scalar: f64) -> Self::Output {
                Self::new(self.quantity / scalar)
            }
        }
    };
}

pub mod length;
pub mod mass;
pub mod measure;
mod speed;
pub mod temp;
pub mod time;

pub use length::lenpriv::{Area, Length, Volume};
pub use speed::Speed;
pub use time::timepriv::{Frequency, Period};
