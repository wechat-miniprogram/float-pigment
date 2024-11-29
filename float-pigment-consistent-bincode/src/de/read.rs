use crate::error::Result;
use crate::io;
use alloc::{boxed::Box, vec::Vec};

/// An optional Read trait for advanced Bincode usage.
///
/// It is highly recommended to use bincode with `io::Read` or `&[u8]` before
/// implementing a custom `BincodeRead`.
///
/// The forward_read_* methods are necessary because some byte sources want
/// to pass a long-lived borrow to the visitor and others want to pass a
/// transient slice.
pub trait BincodeRead<'storage>: io::Read {
    /// Check that the next `length` bytes are a valid string and pass
    /// it on to the serde reader.
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;

    /// Transfer ownership of the next `length` bytes to the caller.
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>>;

    /// Pass a slice of the next `length` bytes on to the serde reader.
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;

    /// Get the current barrier offset.
    fn barrier(&self) -> usize;

    /// Set the barrier to prevent reading beyond.
    fn set_barrier(&mut self, offset: usize);

    /// Skip remaining bytes before the current barrier.
    fn forward_to_barrier(&mut self) -> io::Result<()>;
}

/// A BincodeRead implementation for byte slices
pub struct SliceReader<'storage> {
    slice: &'storage [u8],
    barrier_offset: usize,
}

/// A BincodeRead implementation for `io::Read`ers
pub struct IoReader<R> {
    reader: R,
    temp_buffer: Vec<u8>,
    barrier_offset: usize,
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub(crate) fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader {
            slice: bytes,
            barrier_offset: core::usize::MAX,
        }
    }

    #[inline(always)]
    fn get_byte_slice(&mut self, length: usize) -> Result<&'storage [u8]> {
        if self.barrier_offset < length {
            self.forward_to_barrier()?;
            Err(crate::ErrorKind::SegmentEnded)?;
        }
        self.barrier_offset -= length;
        if length > self.slice.len() {
            return Err(SliceReader::unexpected_eof());
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.slice.is_empty()
    }
}

impl<R> IoReader<R> {
    /// Constructs an IoReadReader
    pub(crate) fn new(r: R) -> IoReader<R> {
        IoReader {
            reader: r,
            temp_buffer: vec![],
            barrier_offset: core::usize::MAX,
        }
    }
}

impl<'storage> io::Read for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.barrier_offset < out.len() {
            self.forward_to_barrier()?;
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        self.barrier_offset -= out.len();
        if out.len() > self.slice.len() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        let (read_slice, remaining) = self.slice.split_at(out.len());
        out.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(out.len())
    }

    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
        self.read(out).map(|_| ())
    }
}

impl<R: io::Read> io::Read for IoReader<R> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.barrier_offset < out.len() {
            self.forward_to_barrier()?;
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        self.barrier_offset -= out.len();
        self.reader.read(out)
    }
    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
        if self.barrier_offset < out.len() {
            self.forward_to_barrier()?;
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        self.barrier_offset -= out.len();
        self.reader.read_exact(out)
    }
}

impl<'storage> SliceReader<'storage> {
    #[inline(always)]
    fn unexpected_eof() -> Box<crate::ErrorKind> {
        Box::new(crate::ErrorKind::Io(io::Error::new(
            io::ErrorKind::UnexpectedEof,
        )))
    }
}

impl<'storage> BincodeRead<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        use crate::ErrorKind;
        let string = match ::core::str::from_utf8(self.get_byte_slice(length)?) {
            Ok(s) => s,
            Err(e) => return Err(ErrorKind::InvalidUtf8Encoding(e).into()),
        };
        visitor.visit_borrowed_str(string)
    }

    #[inline(always)]
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        self.get_byte_slice(length).map(|x| x.to_vec())
    }

    #[inline(always)]
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        visitor.visit_borrowed_bytes(self.get_byte_slice(length)?)
    }

    #[inline(always)]
    fn barrier(&self) -> usize {
        self.barrier_offset
    }

    #[inline(always)]
    fn set_barrier(&mut self, offset: usize) {
        self.barrier_offset = offset;
    }

    #[inline(always)]
    fn forward_to_barrier(&mut self) -> io::Result<()> {
        let (_, remaining) = self.slice.split_at(self.barrier_offset);
        self.slice = remaining;
        self.barrier_offset = 0;
        Ok(())
    }
}

impl<R> IoReader<R>
where
    R: io::Read,
{
    fn fill_buffer(&mut self, length: usize) -> Result<()> {
        if self.barrier_offset < length {
            self.forward_to_barrier()?;
            Err(crate::ErrorKind::SegmentEnded)?;
        }
        self.barrier_offset -= length;

        self.temp_buffer.resize(length, 0);

        self.reader.read_exact(&mut self.temp_buffer)?;

        Ok(())
    }
}

impl<'a, R> BincodeRead<'a> for IoReader<R>
where
    R: io::Read,
{
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;

        let string = match ::core::str::from_utf8(&self.temp_buffer[..]) {
            Ok(s) => s,
            Err(e) => return Err(crate::ErrorKind::InvalidUtf8Encoding(e).into()),
        };

        visitor.visit_str(string)
    }

    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        self.fill_buffer(length)?;
        Ok(core::mem::take(&mut self.temp_buffer))
    }

    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;
        visitor.visit_bytes(&self.temp_buffer[..])
    }

    #[inline(always)]
    fn barrier(&self) -> usize {
        self.barrier_offset
    }

    #[inline(always)]
    fn set_barrier(&mut self, offset: usize) {
        self.barrier_offset = offset;
    }

    #[inline(always)]
    fn forward_to_barrier(&mut self) -> io::Result<()> {
        let mut v = vec![0u8; self.barrier_offset];
        self.barrier_offset = 0;
        self.reader.read_exact(&mut v)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::IoReader;

    #[test]
    fn test_fill_buffer() {
        let buffer = vec![0u8; 64];
        let mut reader = IoReader::new(buffer.as_slice());

        reader.fill_buffer(20).unwrap();
        assert_eq!(20, reader.temp_buffer.len());

        reader.fill_buffer(30).unwrap();
        assert_eq!(30, reader.temp_buffer.len());

        reader.fill_buffer(5).unwrap();
        assert_eq!(5, reader.temp_buffer.len());
    }
}
