#![no_std]
#![allow(clippy::legacy_numeric_constants)]
#![allow(clippy::multiple_bound_locations)]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.  Using it, you can easily go from having
//! an object in memory, quickly serialize it to bytes, and then
//! deserialize it back just as fast!
//!
//! ### Using Basic Functions
//!
//! ```edition2018
//! fn test() {
//!     // The object that we will serialize.
//!     let target: Option<String>  = Some("hello world".to_string());
//!
//!     let encoded: Vec<u8> = float_pigment_consistent_bincode::serialize(&target).unwrap();
//!     let decoded: Option<String> = float_pigment_consistent_bincode::deserialize(&encoded[..]).unwrap();
//!     assert_eq!(target, decoded);
//! }
//! ```
//!
//! ### 128bit numbers
//!
//! Support for `i128` and `u128` is automatically enabled on Rust toolchains
//! greater than or equal to `1.26.0` and disabled for targets which do not support it

#![crate_name = "float_pigment_consistent_bincode"]
#![crate_type = "rlib"]

#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

extern crate byteorder;
#[macro_use]
extern crate serde;

use alloc::vec::Vec;

pub mod config;
/// Deserialize bincode data to a Rust data structure.
pub mod de;
pub mod io;

mod error;
mod internal;
mod ser;

pub use crate::config::{DefaultOptions, Options, SizeDetail};
pub use crate::de::read::BincodeRead;
pub use crate::de::Deserializer;
pub use crate::error::{Error, ErrorKind, Result};
pub use crate::ser::Serializer;

/// Get a default configuration object.
///
/// ### Default Configuration:
///
/// | Byte limit | Endianness | Int Encoding | Trailing Behavior |
/// |------------|------------|--------------|-------------------|
/// | Unlimited  | Little     | Varint       | Reject            |
#[inline(always)]
pub fn options() -> DefaultOptions {
    DefaultOptions::new()
}

/// Serializes an object directly into a `Writer` using the default configuration.
///
/// If the serialization would take more bytes than allowed by the size limit, an error
/// is returned and *no bytes* will be written into the `Writer`.
#[cfg(feature = "std")]
pub fn serialize_into<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: crate::io::Write,
    T: serde::Serialize,
{
    let (_, sizes_list) = DefaultOptions::new().serialized_size(value)?;
    DefaultOptions::new().serialize_into(writer, value, sizes_list)
}

/// Serializes a serializable object into a `Vec` of bytes using the default configuration.
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    DefaultOptions::new().serialize(value)
}

/// Deserializes an object directly from a `Read`er using the default configuration.
///
/// If this returns an `Error`, `reader` may be in an invalid state.
#[cfg(feature = "std")]
pub fn deserialize_from<R, T>(reader: R) -> Result<T>
where
    R: crate::io::Read,
    T: serde::de::DeserializeOwned,
{
    DefaultOptions::new().deserialize_from(reader)
}

/// Deserializes an object from a custom `BincodeRead`er using the default configuration.
/// It is highly recommended to use `deserialize_from` unless you need to implement
/// `BincodeRead` for performance reasons.
///
/// If this returns an `Error`, `reader` may be in an invalid state.
pub fn deserialize_from_custom<'a, R, T>(reader: R) -> Result<T>
where
    R: de::read::BincodeRead<'a>,
    T: serde::de::DeserializeOwned,
{
    DefaultOptions::new().deserialize_from_custom(reader)
}

/// Only use this if you know what you're doing.
///
/// This is part of the public API.
#[doc(hidden)]
pub fn deserialize_in_place<'a, R, T>(reader: R, place: &mut T) -> Result<()>
where
    T: serde::de::Deserialize<'a>,
    R: BincodeRead<'a>,
{
    DefaultOptions::new().deserialize_in_place(reader, place)
}

/// Deserializes a slice of bytes into an instance of `T` using the default configuration.
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    DefaultOptions::new().deserialize(bytes)
}

/// Returns the size that an object would be if serialized using Bincode with the default configuration.
pub fn serialized_size<T: ?Sized>(value: &T) -> Result<(u64, SizeDetail)>
where
    T: serde::Serialize,
{
    DefaultOptions::new().serialized_size(value)
}
