use crate::common::hsl_to_rgb;
use crate::traits::Color;
use crate::{ColorError, Hex, CMYK, HSL, HSLA, HSV, RGB};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// RGBA can be parsed from a string in the format "rgba(r,g,b,a)" or from a tuple (r,g,b,a).
/// * r:u8 - red value(0~255)
/// * g:u8 - green value(0~255)
/// * b:u8 - blue value(0~255)
/// * a:f32 - alpha(0~1)
/// The red, green, and blue values can be individually set or retrieved.
/// ### example
/// ```rust
/// use easy_color::{RGBA, HSL};
/// let mut rgba:RGBA = "rgba(125,60,98,0.8)".try_into().unwrap();
/// rgba.set_alpha(0.5);
/// assert_eq!(rgba.to_string(), "rgba(125,60,98,0.50)");
///
/// let rgba:RGBA = (125,60,240,0.5).try_into().unwrap();
/// let hsl:HSL = rgba.into();
/// assert_eq!(hsl.to_string(), "hsl(262,85%,79%)");
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct RGBA {
    pub(crate) rgb: RGB,
    pub(crate) a: f32,
}

impl TryFrom<&str> for RGBA {
    type Error = ColorError;
    fn try_from(rgba_str: &str) -> Result<Self, Self::Error> {
        let mut color = rgba_str.trim().to_lowercase();
        if color.starts_with("rgba(") && color.ends_with(')') {
            let mut val = vec![];
            let mut alpha = None;
            color = color.replace("rgba(", "").replace(")", "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 4 {
                for (idx, s) in tmp.iter().enumerate() {
                    if idx == 3 {
                        alpha = s.trim().parse::<f32>().ok();
                    } else {
                        if let Ok(v) = s.trim().parse::<u8>() {
                            val.push(v);
                        }
                    }
                }
            }
            if val.len() != 3 || alpha.is_none() {
                return Err(ColorError::FormatErr(format!(
                    "RGBA:{} format error!",
                    rgba_str
                )));
            }
            return (val[0], val[1], val[2], alpha.unwrap()).try_into();
        }
        Err(ColorError::FormatErr(format!(
            "RGBA:{} format error!",
            rgba_str
        )))
    }
}

impl TryFrom<(u8, u8, u8, f32)> for RGBA {
    type Error = ColorError;
    fn try_from(value: (u8, u8, u8, f32)) -> Result<Self, Self::Error> {
        return if !(0.0..=1.0).contains(&value.3) {
            Err(ColorError::ValueErr(format!(
                "RGBA: the alpha value must between 0~1, but got {}.",
                value.3
            )))
        } else {
            let rgb = RGB {
                r: value.0,
                g: value.1,
                b: value.2,
            };
            Ok(RGBA { rgb, a: value.3 })
        };
    }
}

impl From<Hex> for RGBA {
    fn from(hex: Hex) -> Self {
        let (r, g, b, a) = hex.rgba;
        let rgb = RGB { r, g, b };
        Self { rgb, a }
    }
}

impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> Self {
        Self {
            rgb: rgb.clone(),
            a: 1.0,
        }
    }
}

impl From<HSL> for RGBA {
    fn from(hsl: HSL) -> Self {
        let rgb: RGB = hsl.into();
        Self { rgb, a: 1.0 }
    }
}

impl From<HSLA> for RGBA {
    fn from(hsla: HSLA) -> Self {
        let HSLA { hsl, a } = hsla;
        let HSL { h, s, l } = hsl;
        let (r, g, b) = hsl_to_rgb(h, s, l);
        let rgb = RGB { r, g, b };
        Self { rgb, a }
    }
}

impl From<HSV> for RGBA {
    fn from(hsv: HSV) -> Self {
        let rgb: RGB = hsv.into();
        Self { rgb, a: 1.0 }
    }
}

impl From<CMYK> for RGBA {
    fn from(cmyk: CMYK) -> Self {
        let rgb: RGB = cmyk.into();
        Self { rgb, a: 1.0 }
    }
}

impl Deref for RGBA {
    type Target = RGB;
    fn deref(&self) -> &Self::Target {
        &self.rgb
    }
}

impl DerefMut for RGBA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rgb
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RGB { r, g, b } = self.rgb;
        write!(f, "rgba({},{},{},{:.2})", r, g, b, self.a)
    }
}

impl RGBA {
    pub fn alpha(&self) -> f32 {
        self.a
    }
    pub fn set_alpha(&mut self, alpha: f32) -> &mut Self {
        self.a = alpha.max(0.0).min(1.0);
        self
    }
}

impl Color for RGBA {
    fn is_dark(&self) -> bool {
        let rgb = RGB::from(*self);
        rgb.is_dark()
    }

    fn is_light(&self) -> bool {
        !self.is_dark()
    }
}
