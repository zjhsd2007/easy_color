use crate::{RGB, RGBA, HSL, Hex, HSLA, CMYK, HSV};

pub trait Color {
    fn is_dark(&self) -> bool;
    fn is_light(&self) -> bool;
}

impl<T:Into<RGB>+Copy> Color for T {
    fn is_dark(&self) -> bool {
        let agb:RGB=(*self).into();
        agb.is_dark()
    }

    fn is_light(&self) -> bool {
        !self.is_dark()
    }

}

pub trait Grayscale {
    fn grayscale(&self) -> Self; 
}

impl<T: Into<RGBA> + From<RGBA> + Copy >  Grayscale for T {
    fn grayscale(&self) -> Self {
        let rgba:RGBA=(*self).into();
        rgba.grayscale().into()
    }
}

pub trait Negate {
    fn negate(&self) -> Self;
}

impl<T: Into<RGBA> + From<RGBA> + Copy >  Negate for T {
    fn negate(&self) -> Self {
        let rgba:RGBA=(*self).into();
        rgba.negate().into()
    }
}

pub trait ColorMix<T> {
    fn mix(&self, other:T, weight:Option<f32>) -> Self;
}
impl<T:Into<RGBA> + Copy, U: Into<RGBA> + From<RGBA> + Copy> ColorMix<T> for U {
    fn mix(&self, other: T, weight: Option<f32>) -> Self {
        let rgba:RGBA = (*self).into();
        rgba.mix(other, weight).into()
    }
}


pub trait Darken {
    fn darken(&mut self, ratio:f32) -> Self;
}

impl<T:Into<HSL> + From<HSL> + Copy> Darken for T {
    fn darken(&mut self, ratio:f32) -> Self {
        let mut hsl:HSL = (*self).into();
        (*hsl.darken(ratio)).into()
    }
}

pub trait Lighten {
    fn lighten(&mut self, ratio:f32) -> Self;
}

impl<T:Into<HSL> + From<HSL> + Copy> Lighten for T {
    fn lighten(&mut self, ratio:f32) -> Self {
        let mut hsl:HSL = (*self).into();
        (*hsl.lighten(ratio)).into()
    }
}

pub trait IntoHex {
    fn to_hex(&self) -> Hex;
}

impl<T:Into<Hex> + Copy> IntoHex for T {
    fn to_hex(&self) -> Hex {
        (*self).into()
    }
}

pub trait IntoRGB {
    fn to_rgb(&self) -> RGB;
}

impl<T:Into<RGB> + Copy> IntoRGB for T {
    fn to_rgb(&self) -> RGB {
        (*self).into()
    }
}

pub trait IntoRGBA {
    fn to_rgba(&self) -> RGBA;
}

impl<T:Into<RGBA> + Copy> IntoRGBA for T {
    fn to_rgba(&self) -> RGBA {
        (*self).into()
    }
}

pub trait IntoHSL {
    fn to_hsl(&self) -> HSL;
}

impl<T:Into<HSL> + Copy> IntoHSL for T {
    fn to_hsl(&self) -> HSL {
        (*self).into()
    }
}

pub trait IntoHSLA {
    fn to_hsla(&self) -> HSLA;
}

impl<T:Into<HSLA> + Copy> IntoHSLA for T {
    fn to_hsla(&self) -> HSLA {
        (*self).into()
    }
}

pub trait IntoHSV {
    fn to_hsv(&self) -> HSV;
}

impl<T:Into<HSV> + Copy> IntoHSV for T {
    fn to_hsv(&self) -> HSV {
        (*self).into()
    }
}

pub trait IntoCMYK {
    fn to_cmyk(&self) -> CMYK;
}

impl<T:Into<CMYK> + Copy> IntoCMYK for T {
    fn to_cmyk(&self) -> CMYK {
        (*self).into()
    }
}
