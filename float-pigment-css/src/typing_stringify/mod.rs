use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::sheet::borrow::Array;
use crate::typing::*;
use core::fmt;
use cssparser::ToCss;

fn generate_array_str<T: fmt::Display>(array: &Array<T>) -> String {
    let mut str = String::new();
    for index in 0..array.len() {
        str.push_str(&array[index].to_string());
        if index + 1 < array.len() {
            str.push_str(", ");
        }
    }
    str
}

mod background;
mod calc;
mod color;
mod filter;
mod font;
mod layout;
mod misc;
mod shadow;
mod transform;
mod transition;
