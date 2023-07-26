//! A very simple and easy-to-use color conversion tool that can easily convert colors between Hex, RGB, RGBA, HSL, HSLA, HSV, and CMYK.
//! And each type has its unique API, such as RGB type can set color channels, RGBA type can set transparency, HSL type can set hue, saturation, and brightness, etc.
//! ### example:
//! ```rust
//! use easy_color::{RGBA, RGB, HSL, Hex, ColorMix};
//! use crate::easy_color::{IntoRGB, IntoHex, IntoRGBA, IntoHSL, IntoHSLA, IntoHSV, IntoCMYK};
//! let hex:Hex = "#2bc48a".try_into().unwrap();
//!
//! let mut rgb:RGB = hex.into();
//! // or
//! let mut rgb = hex.to_rgb();
//! assert_eq!(rgb.to_string(), "rgb(43,196,138)");
//! rgb.set_red(255);
//! assert_eq!(rgb.to_string(), "rgb(255,196,138)");
//!
//! let mut rgba:RGBA = rgb.into();
//! // or
//! let mut rgba = rgb.to_rgba();
//! rgba.set_alpha(0.5);
//! assert_eq!(rgba.to_string(), "rgba(255,196,138,0.50)");
//!
//! let mut hsl:HSL = rgba.into();
//! // or
//! let mut hsl = rgba.to_hsl();
//! hsl.set_hue(240);
//! assert_eq!(hsl.to_string(), "hsl(240,100%,88%)");
//!
//! let hex:Hex = hsl.into();
//! // or
//! let hex = hsl.to_hex();
//! assert_eq!(hex.to_string(), "#C2C1FF");
//!
//! // mix color
//! let hsl:HSL = (0,0,0).try_into().unwrap();
//! let rgba:RGBA = (255,255,255,1.0).try_into().unwrap();
//! rgba.mix(hsl, None).to_string(); // rgba(127,127,127,1.00)
//! rgba.mix(hsl, Some(0.35)).to_string(); // rgba(165,165,165,1.00)
//! hsl.mix(rgba, None).to_string(); // hsl(0,0%,50%)
//!
//! // creat random color
//! let rgb = RGB::random();
//! let rgba = RGBA::random();
//! let hsl = HSL::random();
//!
//! let hex:Hex = "#2bc48a".try_into().unwrap();
//! let hex_str = hex.to_rgb().set_blue(255).to_hsl().set_lightness(50).to_cmyk().set_cyan(100).to_hex().to_string(); // #00B5FF
//! ```
mod cmyk;
mod common;
mod hex;
mod hsl;
mod hsla;
mod hsv;
mod rgb;
mod rgba;
mod traits;

pub use cmyk::CMYK;
pub use common::ColorError;
pub use hex::Hex;
pub use hsl::HSL;
pub use hsla::HSLA;
pub use hsv::HSV;
pub use rgb::RGB;
pub use rgba::RGBA;
pub use traits::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // hex
        let _hex: Hex = "#FA0".try_into().unwrap();
        let _hex: Hex = "#F7A3B8".try_into().unwrap();
        let _hex: Hex = "#FF99CCD9".try_into().unwrap();
        let hex: Hex = "#2bc48a".try_into().unwrap();

        let rgb: RGB = hex.into();
        assert_eq!(rgb.to_string(), "rgb(43,196,138)");

        let rgba: RGBA = hex.into();
        assert_eq!(rgba.to_string(), "rgba(43,196,138,1.00)");

        let hsl: HSL = hex.into();
        assert_eq!(hsl.to_string(), "hsl(157,64%,47%)");

        let hsla: HSLA = hex.into();
        assert_eq!(hsla.to_string(), "hsla(157,64%,47%,1.00)");

        let hsv: HSV = hex.into();
        assert_eq!(hsv.to_string(), "hsv(157,78%,77%)");

        let cmyk: CMYK = hex.into();
        assert_eq!(cmyk.to_string(), "cmyk(78,0,30,23)");

        // rgb
        let rgb: RGB = "rgb(43,196,138)".try_into().unwrap();
        let hex: Hex = rgb.into();
        assert_eq!(hex.to_string(), "#2BC48A");

        let rgba: RGBA = rgb.into();
        assert_eq!(rgba.to_string(), "rgba(43,196,138,1.00)");

        let hsl: HSL = rgb.into();
        assert_eq!(hsl.to_string(), "hsl(157,64%,47%)");

        let hsla: HSLA = rgb.into();
        assert_eq!(hsla.to_string(), "hsla(157,64%,47%,1.00)");

        let hsv: HSV = rgb.into();
        assert_eq!(hsv.to_string(), "hsv(157,78%,77%)");

        let cmyk: CMYK = rgb.into();
        assert_eq!(cmyk.to_string(), "cmyk(78,0,30,23)");

        //rgba
        let rgba: RGBA = "rgba(43,196,138,0.85)".try_into().unwrap();
        let hex: Hex = rgba.into();
        assert_eq!(hex.to_string(), "#2BC48AD8");

        let rgb: RGB = rgba.into();
        assert_eq!(rgb.to_string(), "rgb(74,204,155)");

        let hsl: HSL = rgba.into();
        assert_eq!(hsl.to_string(), "hsl(157,56%,55%)");

        let hsla: HSLA = rgba.into();
        assert_eq!(hsla.to_string(), "hsla(157,64%,47%,0.85)");

        let hsv: HSV = rgba.into();
        assert_eq!(hsv.to_string(), "hsv(157,64%,80%)");

        let cmyk: CMYK = rgba.into();
        assert_eq!(cmyk.to_string(), "cmyk(64,0,24,20)");

    }
}
