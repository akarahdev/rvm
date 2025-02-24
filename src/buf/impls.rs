use std::io::{Error, ErrorKind};

use super::{ByteBuffer, ReadBuf, WriteBuf};

impl ReadBuf for u8 {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let byte = buf.buf.get(buf.read_idx).ok_or(Error::new(
            ErrorKind::UnexpectedEof,
            "Buffer had unexpected end",
        ))?;
        buf.read_idx += 1;
        Ok(*byte)
    }
}

impl WriteBuf for u8 {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        buf.buf.push(*self);
        Ok(())
    }
}

impl<const N: usize> ReadBuf for [u8; N] {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut value: [u8; N] = [0; N];
        for byte in &mut value {
            *byte = buf.read::<u8>()?;
        }
        Ok(value)
    }
}

impl<const N: usize> WriteBuf for [u8; N] {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        for element in self {
            buf.write(element)?;
        }

        Ok(())
    }
}

impl<T: ReadBuf> ReadBuf for Vec<T> {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = u64::from_le_bytes([
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
        ]);
        let mut value = Vec::with_capacity(len as usize);

        for _ in 0..len {
            value.push(buf.read()?);
        }

        Ok(value)
    }
}

impl<T: WriteBuf> WriteBuf for &[T] {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        let len_bytes = u64::to_le_bytes(self.len() as u64);
        buf.write(&len_bytes)?;

        for value in *self {
            buf.write(value)?;
        }

        Ok(())
    }
}

impl ReadBuf for String {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        String::from_utf8(buf.read::<Vec<u8>>()?)
            .map_err(|_| Error::new(ErrorKind::Unsupported, "String is not utf8"))
    }
}

impl WriteBuf for String {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        buf.write(&self.as_bytes())
    }
}

impl ReadBuf for u64 {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(u64::from_be_bytes([
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
            buf.read()?,
        ]))
    }
}

impl WriteBuf for u64 {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        buf.write(&self.to_be_bytes())
    }
}
