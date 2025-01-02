use crate::config::{BincodeByteOrder, Options};
use crate::io::Read;
use alloc::{boxed::Box, string::String, vec::Vec};

use self::read::{BincodeRead, IoReader, SliceReader};
use crate::config::{IntEncoding, SizeLimit};
use crate::{Error, ErrorKind, Result};
use serde::de::Error as DeError;
use serde::de::IntoDeserializer;

/// Specialized ways to read data into bincode.
pub mod read;

#[derive(Debug, Clone)]
struct SegmentSizeLimit {
    diff: Option<usize>,
}

/// A Deserializer that reads bytes from a buffer.
///
/// This struct should rarely be used.
/// In most cases, prefer the `deserialize_from` function.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to read integers out of the reader.
///
/// ```ignore
/// let d = Deserializer::new(&mut some_reader, SizeLimit::new());
/// serde::Deserialize::deserialize(&mut deserializer);
/// let bytes_read = d.bytes_read();
/// ```
pub struct Deserializer<R, O: Options> {
    pub(crate) reader: R,
    options: O,
    segment_size_limits: Vec<SegmentSizeLimit>,
}

macro_rules! impl_deserialize_literal {
    ($name:ident : $ty:ty = $read:ident()) => {
        #[inline]
        pub(crate) fn $name(&mut self) -> Result<$ty> {
            self.read_literal_type::<$ty>()?;
            self.reader
                .$read::<<O::Endian as BincodeByteOrder>::Endian>()
                .map_err(Into::into)
        }
    };
}

impl<IR: Read, O: Options> Deserializer<IoReader<IR>, O> {
    /// Creates a new Deserializer with a given `Read`er and options.
    pub fn with_reader(r: IR, options: O) -> Self {
        Deserializer {
            reader: IoReader::new(r),
            options,
            segment_size_limits: vec![],
        }
    }
}

impl<'de, O: Options> Deserializer<SliceReader<'de>, O> {
    /// Creates a new Deserializer that will read from the given slice.
    pub fn from_slice(slice: &'de [u8], options: O) -> Self {
        Deserializer {
            reader: SliceReader::new(slice),
            options,
            segment_size_limits: vec![],
        }
    }
}

impl<'de, R: BincodeRead<'de>, O: Options> Deserializer<R, O> {
    /// Creates a new Deserializer with the given `BincodeRead`er
    pub fn with_bincode_read(r: R, options: O) -> Deserializer<R, O> {
        Deserializer {
            reader: r,
            options,
            segment_size_limits: vec![],
        }
    }

    fn begin_segment_size_limit(&mut self) -> Result<()> {
        let seg_size = O::IntEncoding::deserialize_u32(self)? as usize;
        let ssl = SegmentSizeLimit {
            diff: match self.segment_size_limits.last() {
                Some(_) => Some(
                    self.reader
                        .barrier()
                        .checked_sub(seg_size)
                        .ok_or(ErrorKind::InvalidData)?,
                ),
                None => None,
            },
        };
        self.segment_size_limits.push(ssl);
        self.reader.set_barrier(seg_size);
        Ok(())
    }

    fn end_segment_size_limit(&mut self) -> Result<()> {
        if let Some(ssl) = self.segment_size_limits.pop() {
            self.reader.forward_to_barrier()?;
            self.reader
                .set_barrier(ssl.diff.unwrap_or(core::usize::MAX));
        }
        Ok(())
    }

    pub(crate) fn deserialize_byte(&mut self) -> Result<u8> {
        self.read_literal_type::<u8>()?;
        self.reader.read_u8().map_err(Into::into)
    }

    impl_deserialize_literal! { deserialize_literal_u16 : u16 = read_u16() }
    impl_deserialize_literal! { deserialize_literal_u32 : u32 = read_u32() }
    impl_deserialize_literal! { deserialize_literal_u64 : u64 = read_u64() }

    serde_if_integer128! {
        impl_deserialize_literal! { deserialize_literal_u128 : u128 = read_u128() }
    }

    fn read_bytes(&mut self, count: u64) -> Result<()> {
        self.options.limit().add(count)
    }

    fn read_literal_type<T>(&mut self) -> Result<()> {
        use core::mem::size_of;
        self.read_bytes(size_of::<T>() as u64)
    }

    fn read_vec(&mut self) -> Result<Vec<u8>> {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.get_byte_buffer(len)
    }

    fn read_string(&mut self) -> Result<String> {
        let vec = self.read_vec()?;
        String::from_utf8(vec).map_err(|e| ErrorKind::InvalidUtf8Encoding(e.utf8_error()).into())
    }
}

macro_rules! impl_deserialize_int {
    ($name:ident = $visitor_method:ident ($dser_method:ident)) => {
        #[inline]
        fn $name<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'de>,
        {
            visitor.$visitor_method(O::IntEncoding::$dser_method(self)?)
        }
    };
}

impl<'de, 'a, R, O> serde::Deserializer<'de> for &'a mut Deserializer<R, O>
where
    R: BincodeRead<'de>,
    O: Options,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeAnyNotSupported))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.deserialize_byte()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            value => Err(ErrorKind::InvalidBoolEncoding(value).into()),
        }
    }

    impl_deserialize_int!(deserialize_u16 = visit_u16(deserialize_u16));
    impl_deserialize_int!(deserialize_u32 = visit_u32(deserialize_u32));
    impl_deserialize_int!(deserialize_u64 = visit_u64(deserialize_u64));
    impl_deserialize_int!(deserialize_i16 = visit_i16(deserialize_i16));
    impl_deserialize_int!(deserialize_i32 = visit_i32(deserialize_i32));
    impl_deserialize_int!(deserialize_i64 = visit_i64(deserialize_i64));

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_literal_type::<f32>()?;
        let value = self
            .reader
            .read_f32::<<O::Endian as BincodeByteOrder>::Endian>()?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_literal_type::<f64>()?;
        let value = self
            .reader
            .read_f64::<<O::Endian as BincodeByteOrder>::Endian>()?;
        visitor.visit_f64(value)
    }

    serde_if_integer128! {
        impl_deserialize_int!(deserialize_u128 = visit_u128(deserialize_u128));
        impl_deserialize_int!(deserialize_i128 = visit_i128(deserialize_i128));
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.deserialize_byte()?)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.deserialize_byte()? as i8)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        use core::str;

        let error = || ErrorKind::InvalidCharEncoding.into();

        let mut buf = [0u8; 4];

        // Look at the first byte to see how many bytes must be read
        self.reader.read_exact(&mut buf[..1])?;
        let width = utf8_char_width(buf[0]);
        if width == 1 {
            return visitor.visit_char(buf[0] as char);
        }
        if width == 0 {
            return Err(error());
        }

        if self.reader.read_exact(&mut buf[1..width]).is_err() {
            return Err(error());
        }

        let res = str::from_utf8(&buf[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or_else(error)?;
        visitor.visit_char(res)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.forward_read_str(len, visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.forward_read_bytes(len, visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.read_vec()?)
    }

    #[inline(always)]
    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct EnumDeserializer<'a, R, O: Options> {
            de: &'a mut Deserializer<R, O>,
            variants: &'static [&'static str],
            len: u32,
        }

        impl<'de, 'a, R: 'a, O> serde::de::EnumAccess<'de> for EnumDeserializer<'a, R, O>
        where
            R: BincodeRead<'de>,
            O: Options,
        {
            type Error = Error;
            type Variant = &'a mut Deserializer<R, O>;

            #[inline(always)]
            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let idx: u32 = O::IntEncoding::deserialize_u32(self.de)?;
                #[allow(clippy::if_same_then_else)]
                let idx = if idx >= self.len {
                    0
                } else if self.variants[idx as usize] == "_" {
                    0
                } else {
                    idx
                };
                let val: Result<_> = seed.deserialize(idx.into_deserializer());
                Ok((val?, self.de))
            }
        }

        visitor.visit_enum(EnumDeserializer {
            de: self,
            variants,
            len: variants.len() as u32,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options + 'a> {
            deserializer: &'a mut Deserializer<R, O>,
            len: usize,
            _phantom: core::marker::PhantomData<(&'de (), &'b ())>,
        }

        impl<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options> serde::de::SeqAccess<'de>
            for Access<'de, 'a, 'b, R, O>
        {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer);
                    match value {
                        Ok(value) => Ok(Some(value)),
                        Err(err) => match *err {
                            crate::ErrorKind::SegmentEnded => Ok(None),
                            err => Err(Box::new(err)),
                        },
                    }
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
            _phantom: core::marker::PhantomData,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let value: u8 = serde::de::Deserialize::deserialize(&mut *self)?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            v => Err(ErrorKind::InvalidTagEncoding(v as usize).into()),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;

        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options + 'a> {
            deserializer: &'a mut Deserializer<R, O>,
            len: usize,
            _phantom: core::marker::PhantomData<(&'de (), &'b ())>,
        }

        impl<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options> serde::de::MapAccess<'de>
            for Access<'de, 'a, 'b, R, O>
        {
            type Error = Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let key =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                Ok(value)
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        let len = O::IntEncoding::deserialize_len(self)?;

        visitor.visit_map(Access {
            deserializer: self,
            len,
            _phantom: core::marker::PhantomData,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.begin_segment_size_limit()?;
        let r = self.deserialize_tuple(fields.len(), visitor);
        self.end_segment_size_limit()?;
        r
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "Bincode does not support Deserializer::deserialize_identifier";
        Err(Error::custom(message))
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "Bincode does not support Deserializer::deserialize_ignored_any";
        Err(Error::custom(message))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'de, 'a, R, O> serde::de::VariantAccess<'de> for &'a mut Deserializer<R, O>
where
    R: BincodeRead<'de>,
    O: Options,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        self.begin_segment_size_limit()?;
        self.end_segment_size_limit()?;
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.begin_segment_size_limit()?;
        let r = serde::de::DeserializeSeed::deserialize(seed, &mut *self);
        self.end_segment_size_limit()?;
        r
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        use serde::de::Deserializer;
        self.begin_segment_size_limit()?;
        let r = self.deserialize_tuple(len, visitor);
        self.end_segment_size_limit()?;
        r
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        use serde::de::Deserializer;
        self.begin_segment_size_limit()?;
        let r = self.deserialize_tuple(fields.len(), visitor);
        self.end_segment_size_limit()?;
        r
    }
}
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

// This function is a copy of core::str::utf8_char_width
fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
