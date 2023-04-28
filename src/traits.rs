use crate::{RGB, RGBA};

pub trait Color {
    fn is_dark(&self) -> bool;
    fn is_light(&self) -> bool;
}

impl<T:Into<RGB>+Copy> Color for T {
    fn is_dark(&self) -> bool {
        let a:RGB=(*self).into();
        a.is_dark()
    }
    fn is_light(&self) -> bool {
        !self.is_dark()
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
