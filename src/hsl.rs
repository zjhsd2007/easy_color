use crate::common::rgb_to_hsl;
use crate::traits::Color;
use crate::{ColorError, Hex, CMYK, HSLA, HSV, RGB, RGBA};
use std::fmt::{Display, Formatter};

/// HSL can be parsed from a string in the format "hsl(h, s%, l%)" or from a tuple (h,s,l).
/// * h:u32 - Hue(0~360)
/// * s:u32 - saturation(0~100)
/// * l:u32 - lightness(0~100)
/// ### example
/// ```rust
/// use easy_color::{RGB, HSL};
/// let mut hsl:HSL = "hsl(262,85%,79%)".try_into().unwrap();
/// hsl.set_lightness(50);
/// assert_eq!(hsl.to_string(), "hsl(262,85%,50%)");
///
/// let hsl:HSL = (125,60,75).try_into().unwrap();
/// let rgb:RGB = hsl.into();
/// assert_eq!(rgb.to_string(), "rgb(153,229,159)")
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct HSL {
    pub(crate) h: u32,
    pub(crate) s: u32,
    pub(crate) l: u32,
}
impl TryFrom<&str> for HSL {
    type Error = ColorError;
    fn try_from(hsl_str: &str) -> Result<Self, Self::Error> {
        let mut color = hsl_str.trim().to_lowercase();
        if color.starts_with("hsl(") && color.ends_with(')') {
            color = color.replace("hsl(", "").replace(")", "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 3 {
                let val = tmp
                    .iter()
                    .map(|s| s.trim().trim_end_matches('%').parse::<u32>())
                    .filter(|v| v.is_ok())
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>();
                if val.len() == 3 {
                    return (val[0], val[1], val[2]).try_into();
                }
            }
        }
        Err(ColorError::FormatErr(format!(
            "HSL: {} format error!",
            hsl_str
        )))
    }
}

impl TryFrom<(u32, u32, u32)> for HSL {
    type Error = ColorError;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        return if !(0..=360).contains(&value.0)
            || !(0..=100).contains(&value.1)
            || !(0..=100).contains(&value.2)
        {
            Err(ColorError::ValueErr(format!("HSL: args ({},{},{}) value error, first value must between 0~360, others must between 0~100!", value.0, value.1, value.2)))
        } else {
            Ok(Self {
                h: value.0,
                s: value.1,
                l: value.2,
            })
        };
    }
}
impl From<Hex> for HSL {
    fn from(hex: Hex) -> Self {
        let rgba: RGBA = hex.into();
        rgba.into()
    }
}

impl From<RGB> for HSL {
    fn from(rgb: RGB) -> Self {
        let RGB { r, g, b } = rgb;
        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self { h, s, l }
    }
}

impl From<RGBA> for HSL {
    fn from(rgba: RGBA) -> Self {
        let rgb: RGB = rgba.into();
        rgb.into()
    }
}

impl From<HSLA> for HSL {
    fn from(hsla: HSLA) -> Self {
        let rgba: RGBA = hsla.into();
        rgba.into()
    }
}

impl From<HSV> for HSL {
    fn from(hsv: HSV) -> Self {
        let rgb: RGB = hsv.into();
        rgb.into()
    }
}

impl From<CMYK> for HSL {
    fn from(cmyk: CMYK) -> Self {
        let rgb: RGB = cmyk.into();
        rgb.into()
    }
}

impl Display for HSL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsl({},{}%,{}%)", self.h, self.s, self.l)
    }
}

impl HSL {
    pub fn hue(&self) -> u32 {
        self.h
    }

    pub fn set_hue(&mut self, hue: u32) -> &mut Self {
        self.h = hue.min(360);
        self
    }

    pub fn saturation(&self) -> u32 {
        self.s
    }

    pub fn set_saturation(&mut self, saturation: u32) -> &mut Self {
        self.s = saturation.min(100);
        self
    }

    pub fn lightness(&self) -> u32 {
        self.l
    }

    pub fn set_lightness(&mut self, lightness: u32) -> &mut Self {
        self.l = lightness.min(100);
        self
    }
}

impl Color for HSL {
    fn is_dark(&self) -> bool {
        let rgb = RGB::from(*self);
        rgb.is_dark()
    }

    fn is_light(&self) -> bool {
        !self.is_dark()
    }
}
