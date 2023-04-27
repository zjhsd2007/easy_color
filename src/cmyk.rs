use crate::common::{calc_rgb_with_alpha, rgb_to_cmyk};
use crate::traits::Color;
use crate::{ColorError, Hex, HSL, HSLA, HSV, RGB, RGBA};
use std::fmt::{Display, Formatter};

/// CMYK can be parsed from a string in the format "cmyk(c,m,y,k)" or from a tuple (c,m,y,k).
/// * c:u8 - cyan value(0~100)
/// * m:u8 - magenta value(0~100)
/// * y:u8 - yellow value(0~100)
/// * k:u8 - black value(0~100)
/// ### example
/// ```rust
/// use easy_color::{Hex, CMYK};
/// let mut cmyk:CMYK = "cmyk(77,34,53,38)".try_into().unwrap();
/// cmyk.set_cyan(100);
/// assert_eq!(cmyk.to_string(), "cmyk(100,34,53,38)");
///
/// let cmyk:CMYK = (100,34,53,38).try_into().unwrap();
/// let hex:Hex = cmyk.into();
/// assert_eq!(hex.to_string(), "#00684A");
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CMYK {
    pub(crate) c: u8,
    pub(crate) m: u8,
    pub(crate) y: u8,
    pub(crate) k: u8,
}

impl TryFrom<&str> for CMYK {
    type Error = ColorError;
    fn try_from(cmyk_str: &str) -> Result<Self, Self::Error> {
        let mut color = cmyk_str.trim().to_lowercase();
        if color.starts_with("cmyk(") && color.ends_with(')') {
            color = color.replace("cmyk(", "").replace(")", "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 4 {
                let val = tmp
                    .iter()
                    .map(|s| s.parse::<u8>())
                    .filter(|v| v.is_ok())
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>();
                if val.len() == 4 {
                    return (val[0], val[1], val[2], val[3]).try_into();
                }
            }
        }
        Err(ColorError::FormatErr(format!(
            "CMYK: {} format error!",
            cmyk_str
        )))
    }
}

impl TryFrom<(u8, u8, u8, u8)> for CMYK {
    type Error = ColorError;
    fn try_from(value: (u8, u8, u8, u8)) -> Result<Self, Self::Error> {
        return if !(0..=100).contains(&value.0)
            || !(0..=100).contains(&value.1)
            || !(0..=100).contains(&value.2)
            || !(0..=100).contains(&value.3)
        {
            Err(ColorError::ValueErr(format!(
                "CMYK: args ({},{},{},{}) value error. all value must between 0~100",
                value.0, value.1, value.2, value.3
            )))
        } else {
            Ok(Self {
                c: value.0,
                m: value.1,
                y: value.2,
                k: value.3,
            })
        };
    }
}

impl From<Hex> for CMYK {
    fn from(hex: Hex) -> Self {
        let rgba: RGBA = hex.into();
        rgba.into()
    }
}

impl From<RGB> for CMYK {
    fn from(rgb: RGB) -> Self {
        let RGB { r, g, b } = rgb;
        let (c, m, y, k) = rgb_to_cmyk(r, g, b);
        Self { c, m, y, k }
    }
}

impl From<RGBA> for CMYK {
    fn from(rgba: RGBA) -> Self {
        let RGBA { rgb, a } = rgba;
        let RGB { r, g, b } = rgb;
        let r1 = calc_rgb_with_alpha(r, a) as u8;
        let g1 = calc_rgb_with_alpha(g, a) as u8;
        let b1 = calc_rgb_with_alpha(b, a) as u8;
        let (c, m, y, k) = rgb_to_cmyk(r1, g1, b1);
        Self { c, m, y, k }
    }
}

impl From<HSL> for CMYK {
    fn from(hsl: HSL) -> Self {
        let rgb: RGB = hsl.into();
        rgb.into()
    }
}

impl From<HSLA> for CMYK {
    fn from(hsla: HSLA) -> Self {
        let rgba: RGBA = hsla.into();
        rgba.into()
    }
}

impl From<HSV> for CMYK {
    fn from(hsv: HSV) -> Self {
        let rgb: RGB = hsv.into();
        rgb.into()
    }
}

impl Display for CMYK {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cmyk({},{},{},{})", self.c, self.m, self.y, self.k)
    }
}
impl CMYK {
    pub fn cyan(&self) -> u8 {
        self.c
    }
    pub fn set_cyan(&mut self, cyan: u8) -> &mut Self {
        self.c = cyan.min(100);
        self
    }

    pub fn magenta(&self) -> u8 {
        self.m
    }

    pub fn set_magenta(&mut self, magenta: u8) -> &mut Self {
        self.m = magenta.min(100);
        self
    }

    pub fn yellow(&self) -> u8 {
        self.m
    }

    pub fn set_yellow(&mut self, yellow: u8) -> &mut Self {
        self.y = yellow.min(100);
        self
    }

    pub fn black(&self) -> u8 {
        self.m
    }

    pub fn set_black(&mut self, black: u8) -> &mut Self {
        self.m = black.min(100);
        self
    }
}

impl Color for CMYK {
    fn is_dark(&self) -> bool {
        let rgb = RGB::from(*self);
        rgb.is_dark()
    }

    fn is_light(&self) -> bool {
        !self.is_dark()
    }
}
