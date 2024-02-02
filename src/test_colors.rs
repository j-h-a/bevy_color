// Generated by gen_tests. Do not edit.
#[cfg(test)]
use crate::{Hsla, SRgba, LinearRgba, Oklaba};

#[cfg(test)]
pub struct TestColor {
    pub name: &'static str,
    pub rgb: SRgba,
    pub linear_rgb: LinearRgba,
    pub hsl: Hsla,
    pub oklab: Oklaba,
}

// Table of equivalent colors in various color spaces
#[cfg(test)]
pub const TEST_COLORS: &[TestColor] = &[
    // black
    TestColor {
        name: "black",
        rgb: SRgba::new(0.0, 0.0, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
        hsl: Hsla::new(0.0, 0.0, 0.0, 1.0),
        oklab: Oklaba::new(0.0, 0.0, 0.0, 1.0),
    },
    // white
    TestColor {
        name: "white",
        rgb: SRgba::new(1.0, 1.0, 1.0, 1.0),
        linear_rgb: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        hsl: Hsla::new(0.0, 0.0, 1.0, 1.0),
        oklab: Oklaba::new(1.0, 0.0, 0.000000059604645, 1.0),
    },
    // red
    TestColor {
        name: "red",
        rgb: SRgba::new(1.0, 0.0, 0.0, 1.0),
        linear_rgb: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
        hsl: Hsla::new(0.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0),
    },
    // green
    TestColor {
        name: "green",
        rgb: SRgba::new(0.0, 1.0, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
        hsl: Hsla::new(120.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.8664396, -0.2338874, 0.1794985, 1.0),
    },
    // blue
    TestColor {
        name: "blue",
        rgb: SRgba::new(0.0, 0.0, 1.0, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
        hsl: Hsla::new(240.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.4520137, -0.032456964, -0.31152815, 1.0),
    },
    // yellow
    TestColor {
        name: "yellow",
        rgb: SRgba::new(1.0, 1.0, 0.0, 1.0),
        linear_rgb: LinearRgba::new(1.0, 1.0, 0.0, 1.0),
        hsl: Hsla::new(60.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.9679827, -0.07136908, 0.19856972, 1.0),
    },
    // magenta
    TestColor {
        name: "magenta",
        rgb: SRgba::new(1.0, 0.0, 1.0, 1.0),
        linear_rgb: LinearRgba::new(1.0, 0.0, 1.0, 1.0),
        hsl: Hsla::new(300.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.7016738, 0.27456632, -0.16915613, 1.0),
    },
    // cyan
    TestColor {
        name: "cyan",
        rgb: SRgba::new(0.0, 1.0, 1.0, 1.0),
        linear_rgb: LinearRgba::new(0.0, 1.0, 1.0, 1.0),
        hsl: Hsla::new(180.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.90539926, -0.1494439, -0.039398134, 1.0),
    },
    // gray
    TestColor {
        name: "gray",
        rgb: SRgba::new(0.5, 0.5, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.21404114, 0.21404114, 1.0),
        hsl: Hsla::new(0.0, 0.0, 0.5, 1.0),
        oklab: Oklaba::new(0.5981807, 0.00000011920929, 0.0, 1.0),
    },
    // olive
    TestColor {
        name: "olive",
        rgb: SRgba::new(0.5, 0.5, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.21404114, 0.0, 1.0),
        hsl: Hsla::new(60.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.57902855, -0.042691574, 0.11878061, 1.0),
    },
    // purple
    TestColor {
        name: "purple",
        rgb: SRgba::new(0.5, 0.0, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.0, 0.21404114, 1.0),
        hsl: Hsla::new(300.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.41972777, 0.1642403, -0.10118592, 1.0),
    },
    // teal
    TestColor {
        name: "teal",
        rgb: SRgba::new(0.0, 0.5, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.21404114, 0.21404114, 1.0),
        hsl: Hsla::new(180.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.54159236, -0.08939436, -0.02356726, 1.0),
    },
    // maroon
    TestColor {
        name: "maroon",
        rgb: SRgba::new(0.5, 0.0, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.0, 0.0, 1.0),
        hsl: Hsla::new(0.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.3756308, 0.13450874, 0.07527886, 1.0),
    },
    // lime
    TestColor {
        name: "lime",
        rgb: SRgba::new(0.0, 0.5, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.21404114, 0.0, 1.0),
        hsl: Hsla::new(120.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.5182875, -0.13990697, 0.10737252, 1.0),
    },
    // navy
    TestColor {
        name: "navy",
        rgb: SRgba::new(0.0, 0.0, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.0, 0.21404114, 1.0),
        hsl: Hsla::new(240.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.27038592, -0.01941514, -0.18635012, 1.0),
    },
    // orange
    TestColor {
        name: "orange",
        rgb: SRgba::new(0.5, 0.5, 0.0, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.21404114, 0.0, 1.0),
        hsl: Hsla::new(60.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.57902855, -0.042691574, 0.11878061, 1.0),
    },
    // fuchsia
    TestColor {
        name: "fuchsia",
        rgb: SRgba::new(0.5, 0.0, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.21404114, 0.0, 0.21404114, 1.0),
        hsl: Hsla::new(300.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.41972777, 0.1642403, -0.10118592, 1.0),
    },
    // aqua
    TestColor {
        name: "aqua",
        rgb: SRgba::new(0.0, 0.5, 0.5, 1.0),
        linear_rgb: LinearRgba::new(0.0, 0.21404114, 0.21404114, 1.0),
        hsl: Hsla::new(180.0, 1.0, 0.25, 1.0),
        oklab: Oklaba::new(0.54159236, -0.08939436, -0.02356726, 1.0),
    },
];
