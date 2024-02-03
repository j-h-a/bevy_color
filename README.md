# Overview

`bevy_color` is a library of methods to represent and manipulate color values in various
color spaces, compatible with the Bevy game engine.

Color space types:

* `Srgba` - Standard RGB color space.
* `LinearRgba` - Linear RGB color space.
* `Hsla` - Hue, Saturation, Lightness color space.
* `Oklaba` - OKLab color space.

Other types:

* `ColorRepresentation` - an enum that can hold a color of any known type.
* `ColorRange` - a range object that allows interpolation between a start and end color.
* `AnyColorRange` - a type-erased color range that allows generating a gradient or interpolation
  in different color spaces, without exposing details about which color space is being used.
