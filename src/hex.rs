use crate::common::process_hex;
use crate::{ColorError, CMYK, HSL, HSLA, HSV, RGB, RGBA};
use std::fmt::{Display, Formatter};
/// Parse a hexadecimal string into a `Hex` object, which can be converted into `RGB`, `RGBA`, `HSL`, `HSLA`, `HSV`, and `CMYK` objects.
///  ### example
///  ```rust
///  use easy_color::Hex;
///  let _hex:Hex = "#FAC".try_into().unwrap();
///  let _hex:Hex = "#FFDFAC".try_into().unwrap();
///  let _hex:Hex = "#FFDFACDC".try_into().unwrap(); // hex with transparency
///  ```
///
/// Convert hex to other types, such as:
/// ```rust
/// use easy_color::{ Hex, RGBA };
/// let hex:Hex = "#FFDFAC".try_into().unwrap();
/// let mut rgba:RGBA = hex.into();
/// assert_eq!(rgba.to_string(), "rgba(255,223,172,1.00)");
///
/// rgba.set_alpha(0.5);
/// let hex:Hex = rgba.into();
/// let hex_str = hex.to_hex_alpha();
/// assert_eq!(hex_str, "#FFDFAC7F");
///
/// let hex_str2 = hex.to_alpha_hex();
/// assert_eq!(hex_str2, "#7FFFDFAC");
///
///
/// ```
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Hex {
    pub(crate) rgba: (u8, u8, u8, f32),
}

impl TryFrom<&str> for Hex {
    type Error = ColorError;
    fn try_from(hex_str: &str) -> Result<Self, Self::Error> {
        let color = hex_str.trim().to_lowercase();
        if color.starts_with('#') {
            let tmp = color.replace("#", "");
            let len = tmp.len();
            if len == 3 {
                let val = process_hex(tmp.as_str(), 1);
                if val.len() == 3 {
                    return Ok(Self {
                        rgba: (val[0], val[1], val[2], 1.0),
                    });
                }
            }
            if len == 6 {
                let val = process_hex(tmp.as_str(), 2);
                if val.len() == 3 {
                    return Ok(Self {
                        rgba: (val[0], val[1], val[2], 1.0),
                    });
                }
            }

            if len == 8 {
                let val = process_hex(tmp.as_str(), 2);
                if val.len() == 4 {
                    return Ok(Self {
                        rgba: (val[0], val[1], val[2], val[3] as f32 / 255.0),
                    });
                }
            }
        }
        Err(ColorError::FormatErr(format!(
            "'{}' format error!",
            hex_str
        )))
    }
}

impl From<RGB> for Hex {
    fn from(rgb: RGB) -> Self {
        Self {
            rgba: (rgb.r, rgb.g, rgb.b, 1.0),
        }
    }
}
impl From<RGBA> for Hex {
    fn from(rgba: RGBA) -> Self {
        Self {
            rgba: (rgba.r, rgba.g, rgba.b, rgba.a),
        }
    }
}
impl From<HSL> for Hex {
    fn from(hsl: HSL) -> Self {
        let rgb: RGB = hsl.into();
        rgb.into()
    }
}
impl From<HSLA> for Hex {
    fn from(hsla: HSLA) -> Self {
        let rgba: RGBA = hsla.into();
        rgba.into()
    }
}
impl From<HSV> for Hex {
    fn from(hsv: HSV) -> Self {
        let rgb: RGB = hsv.into();
        rgb.into()
    }
}
impl From<CMYK> for Hex {
    fn from(cmyk: CMYK) -> Self {
        let rgb: RGB = cmyk.into();
        rgb.into()
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (r, g, b, a) = self.rgba;
        if self.rgba.3 != 1.0 {
            write!(f, "#{:02X}{:02X}{:02X}{:02X}", r, g, b, (a * 255.0) as u8)
        } else {
            write!(f, "#{:02X}{:02X}{:02X}", r, g, b)
        }
    }
}

impl Hex {
    /// Returns a Hex string with transparency, where the last two characters represent the transparency in hexadecimal.
    /// ```rust
    /// use easy_color::{RGBA, Hex};
    /// let rgba:RGBA = "rgba(255,125,55, 0.85)".try_into().unwrap();
    /// let hex:Hex = rgba.into();
    /// assert_eq!(hex.to_hex_alpha(), "#FF7D37D8");
    /// assert_eq!(hex.to_string(), "#FF7D37D8"); //Returns a Hex string with transparency, where the last two characters represent the transparency in hexadecimal.
    ///
    /// // when alpha value is 1
    /// let rgba:RGBA = "rgba(255,125,55, 1)".try_into().unwrap();
    /// let hex:Hex = rgba.into();
    /// assert_eq!(hex.to_hex_alpha(), "#FF7D37FF");
    /// assert_eq!(hex.to_string(), "#FF7D37");
    /// ```
    pub fn to_hex_alpha(&self) -> String {
        let (r, g, b, a) = self.rgba;
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, (a * 255.0) as u8)
    }

    /// Returns a Hex string with transparency, where the last two characters represent the transparency in hexadecimal.
    ///
    /// ```rust
    /// use easy_color::{RGBA, Hex};
    /// let rgba:RGBA = "rgba(255,125,55, 0.85)".try_into().unwrap();
    /// let hex:Hex = rgba.into();
    /// assert_eq!(hex.to_alpha_hex(), "#D8FF7D37");
    /// ```
    pub fn to_alpha_hex(&self) -> String {
        let (r, g, b, a) = self.rgba;
        format!("#{:02X}{:02X}{:02X}{:02X}", (a * 255.0) as u8, r, g, b)
    }
}