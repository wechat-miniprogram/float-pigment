use alloc::{boxed::Box, string::String, vec::Vec};
use core::cell::Cell;

use crate::sheet::str_store::StrBuffer;

pub trait CompatibilityCheck {
    fn check() {}
}

macro_rules! gen_check {
    ($($x: ty),+) => {
        $(impl CompatibilityCheck for $x {})+
    };
}

macro_rules! gen_check_with_generics {
    ($($x: ty),+) => {
        $(
            impl<T> CompatibilityCheck for $x
                where
                    T: CompatibilityCheck,
                {
                    fn check() {
                        T::check();
                    }
                }
        )+
    };
}

gen_check!(
    (),
    u8,
    f32,
    i32,
    u32,
    u16,
    usize,
    StrBuffer,
    String,
    [f32; 6],
    [f32; 16],
    bool
);

gen_check_with_generics!(Vec<T>, Box<T>, Box<[T]>, Cell<T>, Option<T>);
