//! This is a very simple and easy-to-use color conversion tool that can easily convert colors between Hex, RGB, RGBA, HSL, HSLA, HSV, and CMYK.
//! And each type has its unique API, such as RGB can set color channels, RGBA can set transparency, HSL can set hue, saturation, and brightness, and so on.
//! ### example:
//! ```rust
//! let hex:Hex = "#2bc48a".try_into().unwrap();
//!
//! let mut rgb:RGB = hex.into();
//! assert_eq!(rgb.to_string(), "rgb(43,196,138)");
//! rgb.set_red(255);
//! assert_eq!(rgb.to_string(), "rgb(255,196,138)");
//!
//! let mut rgba:RGBA = rgb.into();
//! rgba.set_alpha(0.5);
//! assert_eq!(rgba.to_string(), "rgba(255,196,138,0.50)");
//!
//! let mut hsl:HSL = rgba.into();
//! hsl.set_hue(240);
//! assert_eq!(hsl.to_string(), "hsl(240,100%,88%)");
//!
//! let hex:Hex = hsl.into();
//! assert_eq!(hex.to_string(), "#C2C1FF");
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
