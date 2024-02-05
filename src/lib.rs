//! Representations of colors in various color spaces.
//!
//! This crate provides a number of color representations, including:
//!
//! - [`SRgba`] (standard RGBA, with gamma correction)
//! - [`LinearRgba`] (linear RGBA, without gamma correction)
//! - [`Hsla`] (hue, saturation, lightness, alpha)
//! - [`Lcha`] (lightness, chroma, hue, alpha)
//! - [`Oklaba`] (hue, chroma, lightness, alpha)
//!
//! Each of these color spaces is represented as distinct Rust types. Colors can be converted
//! from one color space to another using the [`From`] trait.
//!
//! In addition, there is a [`ColorRepresentation`] enum that can represent any of the color
//! types in this crate. This is useful when you need to store a color in a data structure
//! that can't be generic over the color type.
//!
//! # Example
//!
//! ```
//! use bevy_color::{SRgba, Hsla};
//!
//! let srgba = SRgba::new(0.5, 0.2, 0.8, 1.0);
//! let hsla: Hsla = srgba.into();
//!
//! println!("SRgba: {:?}", srgba);
//! println!("Hsla: {:?}", hsla);
//! ```

pub mod color_difference;
mod color_ops;
mod color_range;
mod color_representation;
mod hsla;
mod lcha;
mod linear_rgba;
mod oklaba;
mod srgba;
mod test_colors;
mod testing;
mod to_css_string;

pub use color_ops::*;
pub use color_range::*;
pub use color_representation::*;
pub use hsla::*;
pub use lcha::*;
pub use linear_rgba::*;
pub use oklaba::*;
pub use srgba::*;
pub use to_css_string::*;
