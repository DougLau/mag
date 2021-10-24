// lib.rs
//
// Copyright (C) 2019-2021  Minnesota Department of Transportation
//
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]

// Implement basic ops for a quantity struct
macro_rules! impl_base_ops {
    ($quan:ident, $unit:path) => {
        impl<U> Add for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self::new(self.quantity + other.quantity)
            }
        }
        impl<U> Sub for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self::new(self.quantity - other.quantity)
            }
        }
        impl<U> Mul<f64> for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self::Output {
                Self::new(self.quantity * scalar)
            }
        }
        impl<U> Mul<i32> for $quan<U>
        where
            U: $unit,
        {
            type Output = Self;
            fn mul(self, scalar: i32) -> Self::Output {
                Self::new(self.quantity * f64::from(scalar))
            }
        }
        impl<U> Mul<$quan<U>> for f64
        where
            U: $unit,
        {
            type Output = $quan<U>;
            fn mul(self, other: $quan<U>) -> Self::Output {
                Self::Output::new(self * other.quantity)
            }
        }
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
mod lenpriv;
mod speed;
pub mod temp;
mod temppriv;
pub mod time;
mod timepriv;

pub use lenpriv::{Area, Length, Volume};
pub use speed::Speed;
pub use temppriv::Temperature;
pub use timepriv::{Frequency, Period};
