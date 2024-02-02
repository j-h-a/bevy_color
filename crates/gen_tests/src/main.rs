use palette::{Hsl, IntoColor, LinSrgb, Oklab, Srgb};

const TEST_COLORS: &[(f32, f32, f32, &str)] = &[
    (0., 0., 0., "black"),
    (1., 1., 1., "white"),
    (1., 0., 0., "red"),
    (0., 1., 0., "green"),
    (0., 0., 1., "blue"),
    (1., 1., 0., "yellow"),
    (1., 0., 1., "magenta"),
    (0., 1., 1., "cyan"),
    (0.5, 0.5, 0.5, "gray"),
    (0.5, 0.5, 0., "olive"),
    (0.5, 0., 0.5, "purple"),
    (0., 0.5, 0.5, "teal"),
    (0.5, 0., 0., "maroon"),
    (0., 0.5, 0., "lime"),
    (0., 0., 0.5, "navy"),
    (0.5, 0.5, 0., "orange"),
    (0.5, 0., 0.5, "fuchsia"),
    (0., 0.5, 0.5, "aqua"),
];

fn main() {
    println!(
        "// Generated by gen_tests. Do not edit.
#[cfg(test)]
use crate::{{Hsla, SRgba, LinearRgba, Oklaba}};

#[cfg(test)]
pub struct TestColor {{
    pub name: &'static str,
    pub rgb: SRgba,
    pub linear_rgb: LinearRgba,
    pub hsl: Hsla,
    pub oklab: Oklaba,
}}
"
    );

    println!("// Table of equivalent colors in various color spaces");
    println!("#[cfg(test)]");
    println!("pub const TEST_COLORS: &[TestColor] = &[");
    for (r, g, b, name) in TEST_COLORS {
        let srgb = Srgb::new(*r, *g, *b);
        let linear_rgb: LinSrgb = srgb.into_color();
        let hsl: Hsl = srgb.into_color();
        let oklab: Oklab = srgb.into_color();
        println!("    // {name}");
        println!(
            "    TestColor {{
        name: \"{name}\",
        rgb: SRgba::new({}, {}, {}, 1.0),
        linear_rgb: LinearRgba::new({}, {}, {}, 1.0),
        hsl: Hsla::new({}, {}, {}, 1.0),
        oklab: Oklaba::new({}, {}, {}, 1.0),
    }},",
            VariablePrecision(srgb.red),
            VariablePrecision(srgb.green),
            VariablePrecision(srgb.blue),
            VariablePrecision(linear_rgb.red),
            VariablePrecision(linear_rgb.green),
            VariablePrecision(linear_rgb.blue),
            VariablePrecision(hsl.hue.into_positive_degrees()),
            VariablePrecision(hsl.saturation),
            VariablePrecision(hsl.lightness),
            VariablePrecision(oklab.l),
            VariablePrecision(oklab.a),
            VariablePrecision(oklab.b),
        );
    }
    println!("];");
}

struct VariablePrecision(f32);

impl std::fmt::Display for VariablePrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 {
            return write!(f, "{}.0", self.0);
        }
        write!(f, "{}", self.0)
    }
}
