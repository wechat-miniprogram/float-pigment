use alloc::boxed::Box;

use crate::sheet::{borrow::Array, str_store::StrRef};

pub(crate) trait ResolveFontSize {
    fn resolve_font_size(&mut self, font_size: f32);
}

macro_rules! empty_impl {
    ($x:ty) => {
        impl ResolveFontSize for $x {
            fn resolve_font_size(&mut self, _: f32) {
                // empty
            }
        }
    };
}

empty_impl!(u8);
empty_impl!(u32);
empty_impl!(i32);
empty_impl!(f32);
empty_impl!(StrRef);

impl<T: ResolveFontSize> ResolveFontSize for &mut T {
    fn resolve_font_size(&mut self, font_size: f32) {
        (*self).resolve_font_size(font_size)
    }
}

impl<T: ResolveFontSize> ResolveFontSize for Box<T> {
    fn resolve_font_size(&mut self, font_size: f32) {
        (**self).resolve_font_size(font_size)
    }
}

impl<T: ResolveFontSize> ResolveFontSize for Array<T> {
    fn resolve_font_size(&mut self, font_size: f32) {
        for item in self.iter_mut() {
            item.resolve_font_size(font_size);
        }
    }
}

impl<T: ResolveFontSize, const N: usize> ResolveFontSize for [T; N] {
    fn resolve_font_size(&mut self, font_size: f32) {
        for item in self.iter_mut() {
            item.resolve_font_size(font_size);
        }
    }
}
