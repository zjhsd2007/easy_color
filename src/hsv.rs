use crate::common::{calc_rgb_with_alpha, rgb_to_hsv};
use crate::{ColorError, Hex, CMYK, HSL, HSLA, RGB, RGBA};
use std::fmt::{Display, Formatter};

/// HSV can be parsed from a string in the format "hsl(h, s%, v%)" or from a tuple (h,s,v).
/// * h:u32 - Hue(0~360)
/// * s:u32 - saturation(0~100)
/// * v:u32 - Value(0~100)
/// ### example
/// ```rust
/// use easy_color::{RGB, HSV};
/// let mut hsv:HSV = "hsv(262,85%,79%)".try_into().unwrap();
/// hsv.set_value(50);
/// assert_eq!(hsv.to_string(), "hsv(262,85%,50%)");
///
/// let hsv:HSV = (125,60,75).try_into().unwrap();
/// let rgb:RGB = hsv.into();
/// assert_eq!(rgb.to_string(), "rgb(76,191,86)")
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct HSV {
    pub(crate) h: u32,
    pub(crate) s: u32,
    pub(crate) v: u32,
}

impl TryFrom<&str> for HSV {
    type Error = ColorError;
    fn try_from(hsv_str: &str) -> Result<Self, Self::Error> {
        let mut color = hsv_str.trim().to_lowercase();
        if color.starts_with("hsv(") && color.ends_with(')') {
            color = color.replace("hsv(", "").replace(")", "");
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
            "HSV:{} format error!",
            hsv_str
        )))
    }
}

impl TryFrom<(u32, u32, u32)> for HSV {
    type Error = ColorError;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        return if !(0..=360).contains(&value.0)
            || !(0..=100).contains(&value.1)
            || !(0..=100).contains(&value.2)
        {
            Err(ColorError::ValueErr(format!("HSV: args ({},{},{}) value error. the first value must between 0~360, others must between 0~1.", value.0, value.1, value.2)))
        } else {
            Ok(Self {
                h: value.0,
                s: value.1,
                v: value.2,
            })
        };
    }
}

impl From<Hex> for HSV {
    fn from(hex: Hex) -> Self {
        let rgba: RGBA = hex.into();
        rgba.into()
    }
}

impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        let RGB { r, g, b } = rgb;
        let (h, s, v) = rgb_to_hsv(r, g, b);
        Self { h, s, v }
    }
}

impl From<RGBA> for HSV {
    fn from(rgba: RGBA) -> Self {
        let RGBA { rgb, a } = rgba;
        let RGB { r, g, b } = rgb;
        let r1 = calc_rgb_with_alpha(r, a) as u8;
        let g1 = calc_rgb_with_alpha(g, a) as u8;
        let b1 = calc_rgb_with_alpha(b, a) as u8;
        let (h, s, v) = rgb_to_hsv(r1, g1, b1);
        Self { h, s, v }
    }
}

impl From<HSL> for HSV {
    fn from(hsl: HSL) -> Self {
        let rgb: RGB = hsl.into();
        rgb.into()
    }
}

impl From<HSLA> for HSV {
    fn from(hsla: HSLA) -> Self {
        let rgba: RGBA = hsla.into();
        rgba.into()
    }
}

impl From<CMYK> for HSV {
    fn from(cmyk: CMYK) -> Self {
        let rgb: RGB = cmyk.into();
        rgb.into()
    }
}

impl Display for HSV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsv({},{}%,{}%)", self.h, self.s, self.v)
    }
}
impl HSV {
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

    pub fn value(&self) -> u32 {
        self.v
    }

    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.v = value.min(100);
        self
    }
}