use crate::common::{calc_rgb_with_alpha, cmyk_to_rgb, hsl_to_rgb, hsv_to_rgb};
use crate::{ColorError, Hex, CMYK, HSL, HSLA, HSV, RGBA};
use std::fmt::{Display, Formatter};

/// RGB can be parsed from a string in the format "rgb(r,g,b)" or from a tuple (r,g,b).
/// * r:u8 - red value(0~255)
/// * g:u8 - green value(0~255)
/// * b:u8 - blue value(0~255)
/// The red, green, and blue values can be individually set or retrieved.
/// ### example
/// ```rust
/// use easy_color::{Hex, RGB};
/// let mut rgb:RGB = "rgb(43,196,138)".try_into().unwrap();
/// rgb.set_green(255);
/// assert_eq!(rgb.to_string(), "rgb(43,255,138)");
/// let green = rgb.green(); // 255
///
/// let rgb:RGB = (43, 196, 138).try_into().unwrap();
/// assert_eq!(rgb.to_string(), "rgb(43,196,138)");
///
/// let hex:Hex = rgb.into();
/// assert_eq!(hex.to_string(), "#2BC48A");
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct RGB {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl TryFrom<&str> for RGB {
    type Error = ColorError;
    fn try_from(rgb_str: &str) -> Result<Self, Self::Error> {
        let mut color = rgb_str.trim().to_lowercase();
        if color.starts_with("rgb(") && color.ends_with(')') {
            color = color.replace("rgb(", "").replace(")", "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 3 {
                let val = tmp
                    .iter()
                    .map(|s| s.trim().parse::<u8>())
                    .filter(|v| v.is_ok())
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>();
                if val.len() == 3 {
                    return (val[0], val[1], val[2]).try_into();
                }
            }
        }
        Err(ColorError::FormatErr(format!(
            "RGB:{} format error!",
            rgb_str
        )))
    }
}

impl TryFrom<(u8, u8, u8)> for RGB {
    type Error = ColorError;
    fn try_from(value: (u8, u8, u8)) -> Result<Self, Self::Error> {
        Ok(RGB {
            r: value.0,
            g: value.1,
            b: value.2,
        })
    }
}

impl From<Hex> for RGB {
    fn from(hex: Hex) -> Self {
        let (r, g, b, a) = hex.rgba;
        let r = calc_rgb_with_alpha(r, a) as u8;
        let g = calc_rgb_with_alpha(g, a) as u8;
        let b = calc_rgb_with_alpha(b, a) as u8;
        (r, g, b).try_into().unwrap()
    }
}

impl From<RGBA> for RGB {
    fn from(rgba: RGBA) -> Self {
        let r = calc_rgb_with_alpha(rgba.r, rgba.a) as u8;
        let g = calc_rgb_with_alpha(rgba.g, rgba.a) as u8;
        let b = calc_rgb_with_alpha(rgba.b, rgba.a) as u8;
        (r, g, b).try_into().unwrap()
    }
}

impl From<HSL> for RGB {
    fn from(hsl: HSL) -> Self {
        let HSL { h, s, l } = hsl;
        let (r, g, b) = hsl_to_rgb(h, s, l);
        Self { r, g, b }
    }
}

impl From<HSLA> for RGB {
    fn from(hsla: HSLA) -> Self {
        let HSLA { hsl, a } = hsla;
        let HSL { h, s, l } = hsl;
        let (mut r, mut g, mut b) = hsl_to_rgb(h, s, l);
        r = calc_rgb_with_alpha(r, a) as u8;
        g = calc_rgb_with_alpha(g, a) as u8;
        b = calc_rgb_with_alpha(b, a) as u8;
        Self { r, g, b }
    }
}

impl From<HSV> for RGB {
    fn from(hsv: HSV) -> Self {
        let HSV { h, s, v } = hsv;
        let (r, g, b) = hsv_to_rgb(h, s, v);
        Self { r, g, b }
    }
}

impl From<CMYK> for RGB {
    fn from(cmyk: CMYK) -> Self {
        let CMYK { c, m, y, k } = cmyk;
        let (r, g, b) = cmyk_to_rgb(c, m, y, k);
        Self { r, g, b }
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

impl RGB {
    pub fn red(&self) -> u8 {
        self.r
    }
    pub fn set_red(&mut self, red: u8) -> &mut Self {
        self.r = red.min(255);
        self
    }
    pub fn green(&self) -> u8 {
        self.g
    }
    pub fn set_green(&mut self, green: u8) -> &mut Self {
        self.g = green.min(255);
        self
    }
    pub fn blue(&self) -> u8 {
        self.b
    }
    pub fn set_blue(&mut self, blue: u8) -> &mut Self {
        self.b = blue.min(255);
        self
    }

    pub fn is_dark(&self) -> bool {
        self.r as f32 * 0.299 + self.g as f32 * 0.587 + self.b as f32 * 0.114 < 192.0
    }

    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }
}