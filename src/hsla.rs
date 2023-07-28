use crate::common::{rgb_to_hsl, rgba_to_hsla};
use crate::{ColorError, Hex, CMYK, HSL, HSV, RGB, RGBA};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// HSLA can be parsed from a string in the format "hsla(h, s%, l%, a)" or from a tuple (h,s,l,a).
/// * h:u32 - Hue(0~360)
/// * s:u32 - saturation(0~100)
/// * l:u32 - lightness(0~100)
/// * a:f32 - alpha(0~1)
/// ### example
/// ```rust
/// use easy_color::{RGBA, HSLA};
/// let mut hsla:HSLA = "hsla(262,85%,79%, 0.7)".try_into().unwrap();
/// hsla.set_alpha(0.5);
/// assert_eq!(hsla.to_string(), "hsla(262,85%,79%,0.50)");
///
/// let hsla:HSLA = (125,60,75,0.6).try_into().unwrap();
/// let rgba:RGBA = hsla.into();
/// assert_eq!(rgba.to_string(), "rgba(153,229,159,0.60)");
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct HSLA {
    pub(crate) hsl: HSL,
    pub(crate) a: f32,
}

impl TryFrom<&str> for HSLA {
    type Error = ColorError;
    fn try_from(hsla_str: &str) -> Result<Self, Self::Error> {
        let mut color = hsla_str.trim().to_lowercase();
        if color.starts_with("hsla(") && color.ends_with(')') {
            let mut val = vec![];
            let mut alpha = None;
            color = color.replace("hsla(", "").replace(')', "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 4 {
                for (idx, s) in tmp.iter().enumerate() {
                    if idx == 3 {
                        alpha = s.trim().parse::<f32>().ok();
                    } else if let Ok(v) = s.trim().trim_end_matches('%').parse::<u32>() {
                        val.push(v);
                    }
                }
            }
            if let Some(alpha) = alpha {
                if val.len() == 3 {
                    return (val[0], val[1], val[2], alpha).try_into();
                }
            }
        }
        Err(ColorError::FormatErr(format!(
            "HSLA: {} format error!",
            hsla_str
        )))
    }
}

impl TryFrom<(u32, u32, u32, f32)> for HSLA {
    type Error = ColorError;
    fn try_from(value: (u32, u32, u32, f32)) -> Result<Self, Self::Error> {
        if !(0..=360).contains(&value.0)
            || !(0..=100).contains(&value.1)
            || !(0..=100).contains(&value.2)
            || !(0.0..=1.0).contains(&value.3)
        {
            Err(ColorError::ValueErr(format!("HSLA: args ({},{},{},{}) value error. first value must between 0~360, second and third must between 0~100, and last one must between 0~1", value.0, value.1, value.2, value.3)))
        } else {
            let hsl = HSL {
                h: value.0,
                s: value.1,
                l: value.2,
            };
            Ok(HSLA { hsl, a: value.3 })
        }
    }
}
impl From<Hex> for HSLA {
    fn from(hex: Hex) -> Self {
        let rgba: RGBA = hex.into();
        rgba.into()
    }
}

impl From<RGB> for HSLA {
    fn from(rgb: RGB) -> Self {
        let RGB { r, g, b } = rgb;
        let (h, s, l) = rgb_to_hsl(r, g, b);
        let hsl = HSL { h, s, l };
        Self { hsl, a: 1.0 }
    }
}

impl From<RGBA> for HSLA {
    fn from(rgba: RGBA) -> Self {
        let RGBA { rgb, a } = rgba;
        let RGB { r, g, b } = rgb;
        let (h, s, l, a) = rgba_to_hsla(r, g, b, a);
        let hsl = HSL { h, s, l };
        Self { hsl, a }
    }
}

impl From<HSL> for HSLA {
    fn from(hsl: HSL) -> Self {
        Self { hsl, a: 1.0 }
    }
}

impl From<HSV> for HSLA {
    fn from(hsv: HSV) -> Self {
        let rgb: RGB = hsv.into();
        rgb.into()
    }
}

impl From<CMYK> for HSLA {
    fn from(cmyk: CMYK) -> Self {
        let rgb: RGB = cmyk.into();
        rgb.into()
    }
}

impl Deref for HSLA {
    type Target = HSL;
    fn deref(&self) -> &Self::Target {
        &self.hsl
    }
}

impl DerefMut for HSLA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hsl
    }
}

impl Display for HSLA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let HSL { h, s, l } = self.hsl;
        write!(f, "hsla({},{}%,{}%,{:.2})", h, s, l, self.a)
    }
}
impl HSLA {
    pub fn alpha(&self) -> f32 {
        self.a
    }

    pub fn set_alpha(&mut self, alpha: f32) -> &mut Self {
        self.a = alpha.max(0.0).min(1.0);
        self
    }

    /// Generate HSLA, value is random
    pub fn random() -> Self {
        let hsl = HSL::random();
        let a = (rand::random::<f32>() * 100.0_f32).round() / 100.0;
        Self { hsl, a }
    }
}
