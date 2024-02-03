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
