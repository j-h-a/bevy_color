use crate::{
    to_css_string::{RoundToDecimalPlaces, ToCssString},
    LinearRgba, LuminanceOps, Mix, SRgba, WithAlpha,
};
use bevy::render::color::{Color, LchRepresentation};
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Color in Oklaba color space, with alpha
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct Lcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}

impl Lcha {
    /// Construct a new [`Oklaba`] color from components.
    ///
    /// # Arguments
    ///
    /// * `lightness` - Lightness channel. [0.0, 1.5]
    /// * `chroma` - Chroma channel. [0.0, 1.5]
    /// * `hue` - Hue channel. [0.0, 360.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Self {
        Self {
            lightness,
            chroma,
            hue,
            alpha,
        }
    }

    /// Convert the Lcha color to a tuple of components (lightness, chroma, hue, alpha). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.lightness, self.chroma, self.hue, self.alpha)
    }

    /// Construct a new [`Lcha`] color from a tuple of components (lightness, chroma, hue, alpha).
    #[inline]
    pub const fn from_components((lightness, chroma, hue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(lightness, chroma, hue, alpha)
    }
}

impl Default for Lcha {
    fn default() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}

impl ToCssString for Lcha {
    fn to_css_string(&self) -> String {
        format!(
            "color(lch {}% {} {} {})",
            (self.lightness * 100.0).round_to_decimal_places(3),
            self.chroma.round_to_decimal_places(6),
            self.hue.round_to_decimal_places(6),
            self.alpha
        )
    }
}

impl Mix for Lcha {
    #[inline]
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        Self {
            lightness: self.lightness * n_factor + other.lightness * factor,
            chroma: self.chroma * n_factor + other.chroma * factor,
            hue: self.hue * n_factor + other.hue * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }
}

impl WithAlpha for Lcha {
    #[inline]
    fn with_alpha(&self, alpha: f32) -> Self {
        Self { alpha, ..*self }
    }
}

impl LuminanceOps for Lcha {
    #[inline]
    fn with_luminance(&self, lightness: f32) -> Self {
        Self { lightness, ..*self }
    }

    fn luminance(&self) -> f32 {
        self.lightness
    }

    fn darker(&self, amount: f32) -> Self {
        Self::new(
            (self.lightness - amount).max(0.),
            self.chroma,
            self.hue,
            self.alpha,
        )
    }

    fn lighter(&self, amount: f32) -> Self {
        Self::new(
            (self.lightness + amount).min(1.),
            self.chroma,
            self.hue,
            self.alpha,
        )
    }
}

impl From<SRgba> for Lcha {
    fn from(value: SRgba) -> Self {
        let (l, c, h) =
            LchRepresentation::nonlinear_srgb_to_lch([value.red, value.green, value.blue]);
        Lcha::new(l, c, h, value.alpha)
    }
}

impl From<Lcha> for SRgba {
    fn from(value: Lcha) -> Self {
        let [r, g, b] =
            LchRepresentation::lch_to_nonlinear_srgb(value.lightness, value.chroma, value.hue);
        SRgba::new(r, g, b, value.alpha)
    }
}

impl From<LinearRgba> for Lcha {
    fn from(value: LinearRgba) -> Self {
        SRgba::from(value).into()
    }
}

impl From<Lcha> for LinearRgba {
    fn from(value: Lcha) -> Self {
        LinearRgba::from(SRgba::from(value))
    }
}

impl From<Lcha> for Color {
    fn from(value: Lcha) -> Self {
        Color::Lcha {
            hue: value.hue,
            chroma: value.chroma,
            lightness: value.lightness,
            alpha: value.alpha,
        }
    }
}

impl From<Color> for Lcha {
    fn from(value: Color) -> Self {
        match value.as_lcha() {
            Color::Lcha {
                hue,
                chroma,
                lightness,
                alpha,
            } => Lcha::new(hue, chroma, lightness, alpha),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color_difference::EuclideanDistance, test_colors::TEST_COLORS, testing::assert_approx_eq,
        SRgba,
    };

    #[test]
    fn test_to_from_srgba() {
        for color in TEST_COLORS.iter() {
            let rgb2: SRgba = (color.lch).into();
            let lcha: Lcha = (color.rgb).into();
            assert!(
                color.rgb.distance(&rgb2) < 0.0001,
                "{}: {:?} != {:?}",
                color.name,
                color.rgb,
                rgb2
            );
            // For some reason hue is off by 1.7 degrees. This is a difference between
            // Bevy's Lch and the Lch used in the test data generated by `palette`.
            assert_approx_eq!(color.lch.lightness, lcha.lightness, 0.001);
            assert_approx_eq!(color.lch.chroma, lcha.chroma, 0.1);
            assert_approx_eq!(color.lch.hue, lcha.hue, 1.7);
            assert_approx_eq!(color.lch.alpha, lcha.alpha, 0.001);
        }
    }

    #[test]
    fn test_to_from_linear() {
        for color in TEST_COLORS.iter() {
            let rgb2: LinearRgba = (color.lch).into();
            let lcha: Lcha = (color.linear_rgb).into();
            assert!(
                color.linear_rgb.distance(&rgb2) < 0.0001,
                "{}: {:?} != {:?}",
                color.name,
                color.linear_rgb,
                rgb2
            );
            // For some reason hue is off by 1.7 degrees. This is a difference between
            // Bevy's Lch and the Lch used in the test data generated by `palette`.
            assert_approx_eq!(color.lch.lightness, lcha.lightness, 0.001);
            assert_approx_eq!(color.lch.chroma, lcha.chroma, 0.1);
            assert_approx_eq!(color.lch.hue, lcha.hue, 1.8);
            assert_approx_eq!(color.lch.alpha, lcha.alpha, 0.001);
        }
    }

    #[test]
    fn to_css_string() {
        assert_eq!(
            Lcha::from(SRgba::WHITE).to_css_string(),
            "color(lch 100% 0 0 1)"
        );
        assert_eq!(
            Lcha::from(SRgba::RED).to_css_string(),
            "color(lch 53.241% 1.045518 39.99901 1)"
        );
        assert_eq!(
            Lcha::from(SRgba::NONE).to_css_string(),
            "color(lch 0% 0 0 0)"
        );
    }
}
