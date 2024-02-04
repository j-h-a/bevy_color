/// Operations on a color.
pub trait ColorOps: Sized {
    /// Return the luminance of this color (0.0 - 1.0).
    fn luminance(&self) -> f32;

    /// Return the saturation of this color (0.0 - 1.0).
    fn saturation(&self) -> f32;

    /// Return a darker version of this color. The `amount` should be between 0.0 and 1.0, and
    /// is not relative to the current color, but rather represents fixed steps. So for example,
    /// if the color is a 50% gray, and `amount` is 0.25, the result will be 25% gray.
    fn darken(&self, amount: f32) -> Self;

    /// Return a lighter version of this color. The `amount` should be between 0.0 and 1.0, and
    /// is not relative to the current color, but rather represents fixed steps. So for example,
    /// if the color is a 50% gray, and `amount` is 0.25, the result will be 75% gray.
    fn lighten(&self, amount: f32) -> Self;
}

/// Linear interpolation of two colors within a given color space.
pub trait Mix: Sized {
    /// Linearly interpolate between this and another color, by factor.
    /// Factor should be between 0.0 and 1.0.
    fn mix(&self, other: &Self, factor: f32) -> Self;

    /// Linearly interpolate between this and another color, by factor, storing the result
    /// in this color. Factor should be between 0.0 and 1.0.
    fn mix_assign(&mut self, other: Self, factor: f32) {
        *self = self.mix(&other, factor);
    }
}

/// Methods for manipulating alpha values.
pub trait WithAlpha: Sized {
    /// Return a new version of this color with the given alpha value.
    fn with_alpha(&self, alpha: f32) -> Self;
}

/// Methods for changing the luminance of a color.
pub trait WithLuminance: Sized {
    /// Return a new version of this color with the given alpha value.
    fn with_luminance(&self, alpha: f32) -> Self;
}
