use crate::{RGB, RGBA, HSL};

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
    fn negate(&self) -> Self;
}

impl<T: Into<RGBA> + From<RGBA> + Copy >  Grayscale for T {
    fn grayscale(&self) -> Self {
        let rgba:RGBA=(*self).into();
        rgba.grayscale().into()
    }
    
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