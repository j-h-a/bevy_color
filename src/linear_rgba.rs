use crate::{
    color_difference::EuclideanDistance, oklaba::Oklaba, to_css_string::ToCssString, Hsla,
    LuminanceOps, Mix, SRgba, WithAlpha,
};
use bevy::{
    math::Vec4,
    render::color::{Color, SrgbColorSpace},
};
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Linear standard RGB color with alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct LinearRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl LinearRgba {
    /// Construct a new LinearRgba color from components.
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Convert the [`LinearRgba`] color to a tuple of components (r, g, b, a). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Construct a new [`LinearRgba`] color from a tuple of components (r, g, b, a).
    #[inline]
    pub const fn from_components((red, green, blue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(red, green, blue, alpha)
    }

    /// Convert the [`LinearRgba`] color to a [`SRgba`].
    pub fn to_nonlinear(&self) -> SRgba {
        SRgba::from(*self)
    }

    /// Make the color lighter or darker by some amount
    fn adjust_lightness(&mut self, amount: f32) {
        let luminance = self.luminance();
        let target_luminance = (luminance + amount).clamp(0.0, 1.0);
        if target_luminance < luminance {
            let adjustment = (luminance - target_luminance) / luminance;
            self.mix_assign(Self::new(0.0, 0.0, 0.0, self.alpha), adjustment);
        } else if target_luminance > luminance {
            let adjustment = (target_luminance - luminance) / (1. - luminance);
            self.mix_assign(Self::new(1.0, 1.0, 1.0, self.alpha), adjustment);
        }
    }
}

impl Default for LinearRgba {
    fn default() -> Self {
        Self {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 1.,
        }
    }
}

impl ToCssString for LinearRgba {
    fn to_css_string(&self) -> String {
        format!(
            "color(srgb-linear {} {} {} {})",
            self.red * 255.0,
            self.green * 255.0,
            self.blue * 255.0,
            self.alpha
        )
    }
}

impl LuminanceOps for LinearRgba {
    #[inline]
    fn luminance(&self) -> f32 {
        self.red * 0.2126 + self.green * 0.7152 + self.blue * 0.0722
    }

    #[inline]
    fn with_luminance(&self, luminance: f32) -> Self {
        let current_luminance = self.red * 0.2126 + self.green * 0.7152 + self.blue * 0.0722;
        let adjustment = luminance / current_luminance;
        Self {
            red: (self.red * adjustment).clamp(0., 1.),
            green: self.green * adjustment.clamp(0., 1.),
            blue: self.blue * adjustment.clamp(0., 1.),
            alpha: self.alpha,
        }
    }

    #[inline]
    fn darker(&self, amount: f32) -> Self {
        let mut result = *self;
        result.adjust_lightness(-amount);
        result
    }

    #[inline]
    fn lighter(&self, amount: f32) -> Self {
        let mut result = *self;
        result.adjust_lightness(amount);
        result
    }
}

impl Mix for LinearRgba {
    #[inline]
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        Self {
            red: self.red * n_factor + other.red * factor,
            green: self.green * n_factor + other.green * factor,
            blue: self.blue * n_factor + other.blue * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }
}

impl WithAlpha for LinearRgba {
    #[inline]
    fn with_alpha(&self, alpha: f32) -> Self {
        Self { alpha, ..*self }
    }
}

impl EuclideanDistance for LinearRgba {
    #[inline]
    fn distance_squared(&self, other: &Self) -> f32 {
        let dr = self.red - other.red;
        let dg = self.green - other.green;
        let db = self.blue - other.blue;
        dr * dr + dg * dg + db * db
    }
}

impl From<SRgba> for LinearRgba {
    #[inline]
    fn from(value: SRgba) -> Self {
        Self {
            red: value.red.nonlinear_to_linear_srgb(),
            green: value.green.nonlinear_to_linear_srgb(),
            blue: value.blue.nonlinear_to_linear_srgb(),
            alpha: value.alpha,
        }
    }
}

impl From<LinearRgba> for Color {
    fn from(value: LinearRgba) -> Self {
        Color::RgbaLinear {
            red: value.red,
            green: value.green,
            blue: value.blue,
            alpha: value.alpha,
        }
    }
}

impl From<Color> for LinearRgba {
    fn from(value: Color) -> Self {
        match value.as_rgba_linear() {
            Color::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            } => LinearRgba::new(red, green, blue, alpha),
            _ => unreachable!(),
        }
    }
}

impl From<LinearRgba> for [f32; 4] {
    fn from(color: LinearRgba) -> Self {
        [color.red, color.green, color.blue, color.alpha]
    }
}

impl From<LinearRgba> for Vec4 {
    fn from(color: LinearRgba) -> Self {
        Vec4::new(color.red, color.green, color.blue, color.alpha)
    }
}

#[allow(clippy::excessive_precision)]
impl From<Oklaba> for LinearRgba {
    fn from(value: Oklaba) -> Self {
        let Oklaba { l, a, b, alpha } = value;

        // From https://github.com/Ogeon/palette
        let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let red = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let green = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let blue = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<Hsla> for LinearRgba {
    #[inline]
    fn from(value: Hsla) -> Self {
        LinearRgba::from(SRgba::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_distance() {
        // White to black
        let a = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
        let b = LinearRgba::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Alpha shouldn't matter
        let a = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
        let b = LinearRgba::new(1.0, 1.0, 1.0, 0.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Red to green
        let a = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
        let b = LinearRgba::new(1.0, 0.0, 0.0, 1.0);
        assert_eq!(a.distance_squared(&b), 1.0);
    }

    #[test]
    fn to_css_string() {
        assert_eq!(
            LinearRgba::from(SRgba::WHITE).to_css_string(),
            "color(srgb-linear 255 255 255 1)"
        );
        assert_eq!(
            LinearRgba::from(SRgba::RED).to_css_string(),
            "color(srgb-linear 255 0 0 1)"
        );
        assert_eq!(
            LinearRgba::from(SRgba::NONE).to_css_string(),
            "color(srgb-linear 0 0 0 0)"
        );
    }

    #[test]
    fn darker_lighter() {
        // Darker and lighter should be commutative.
        let color = LinearRgba::new(0.4, 0.5, 0.6, 1.0);
        let darker1 = color.darker(0.1);
        let darker2 = darker1.darker(0.1);
        let twice_as_dark = color.darker(0.2);
        assert!(darker2.distance_squared(&twice_as_dark) < 0.0001);

        let lighter1 = color.lighter(0.1);
        let lighter2 = lighter1.lighter(0.1);
        let twice_as_light = color.lighter(0.2);
        assert!(lighter2.distance_squared(&twice_as_light) < 0.0001);
    }
}
