use crate::common::hsl_to_rgb;
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
            color = color.replace("rgba(", "").replace(')', "");
            let tmp = color.split(',').collect::<Vec<_>>();
            if tmp.len() == 4 {
                for (idx, s) in tmp.iter().enumerate() {
                    if idx == 3 {
                        alpha = s.trim().parse::<f32>().ok();
                    } else if let Ok(v) = s.trim().parse::<u8>() {
                        val.push(v);
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
        if !(0.0..=1.0).contains(&value.3) {
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
        }
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
        Self { rgb, a: 1.0 }
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

    /// mix color
    /// ### Arguments
    /// * other - any struct that impl into RGBA
    /// * weight: Option<f32> the mixed color`s weight
    /// ### Example
    /// ```rust
    /// use easy_color::{HSL, RGBA, ColorMix};
    /// let hsl:HSL = (0,0,0).try_into().unwrap();
    /// let rgba:RGBA = (255,255,255,1.0).try_into().unwrap();
    /// rgba.mix(hsl, None).to_string(); // rgba(127,127,127,1.00)
    /// rgba.mix(hsl, Some(0.35)).to_string(); // rgba(165,165,165,1.00)
    /// hsl.mix(rgba, None).to_string(); // hsl(0,0%,50%)
    /// ```
    ///
    pub fn mix(&self, other: impl Into<Self>, weight: Option<f32>) -> Self {
        let rgba: RGBA = other.into();
        let p = weight.unwrap_or(0.5);
        let w = 2.0 * p - 1.0;
        let a = rgba.a - self.a;
        let w1 = if w * a == -1.0 {
            (w + 1.0) / 2.0
        } else {
            ((w + a) / (1.0 + w * a) + 1.0) / 2.0
        };
        let w2 = 1.0 - w1;
        let r = (w1 * rgba.r as f32 + w2 * self.r as f32) as u8;
        let g = (w1 * rgba.g as f32 + w2 * self.g as f32) as u8;
        let b = (w1 * rgba.b as f32 + w2 * self.b as f32) as u8;
        let a = rgba.a * p + self.a * (1.0 - p);
        let rgb: RGB = (r, g, b).try_into().unwrap();
        Self { rgb, a }
    }

    /// fade color
    /// * ratio:f32 - the ratio of fading, a value between 0.0 and 1.0
    ///
    /// This method reduces the alpha value of the color by the given ratio, making it more transparent.
    /// The resulting alpha value is clamped between 0.0 and 1.0.
    /// ``` rust
    /// use easy_color::RGBA;
    /// let mut rgba:RGBA = (255,255,255,0.8).try_into().unwrap();
    /// rgba.fade(0.5);
    /// assert_eq!(rgba.to_string(), "rgba(255,255,255,0.40)");
    /// ```
    pub fn fade(&mut self, ratio: f32) -> &mut Self {
        self.a = (self.a - self.a * ratio).max(0.0).min(1.0);
        self
    }

    /// Increase or decrease the opacity of the color by the given ratio, making it more or less opaque.
    /// The resulting alpha value is clamped between 0.0 and 1.0.
    /// * ratio:f32 - the ratio of opacity change, a positive value increases opacity, a negative value decreases opacity.
    ///
    /// ``` rust
    /// use easy_color::RGBA;
    /// let mut rgba:RGBA = (255,255,255,0.8).try_into().unwrap();
    /// rgba.opaquer(0.2);
    /// assert_eq!(rgba.to_string(), "rgba(255,255,255,0.96)");
    /// ```
    pub fn opaquer(&mut self, ratio: f32) -> &mut Self {
        self.a = (self.a + self.a * ratio).max(0.0).min(1.0);
        self
    }

    /// Returns the grayscale mode of the color
    /// ``` rust
    /// use easy_color::RGBA;
    /// let rgba:RGBA = (95,45,155,0.8).try_into().unwrap();
    /// let gray = rgba.grayscale();
    /// assert_eq!(gray.to_string(), "rgba(72,72,72,0.80)");
    /// ```
    pub fn grayscale(&self) -> Self {
        let v = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8;
        (v, v, v, self.a).try_into().unwrap()
    }

    /// Invert color
    /// ```rust
    /// use easy_color::RGBA;
    /// let rgba:RGBA = (95,45,155,0.8).try_into().unwrap();
    /// let inverted = rgba.negate();
    /// assert_eq!(inverted.to_string(), "rgba(160,210,100,0.80)");
    /// ```
    pub fn negate(&self) -> Self {
        let RGB {
            mut r,
            mut g,
            mut b,
        } = self.rgb;
        r = 255 - r;
        g = 255 - g;
        b = 255 - b;
        let rgb: RGB = (r, g, b).try_into().unwrap();
        Self { rgb, a: self.a }
    }

    pub fn random() -> Self {
        let rgb = RGB::random();
        let a = (rand::random::<f32>() * 100.0_f32).round() / 100.0;
        Self { rgb, a }
    }
}
