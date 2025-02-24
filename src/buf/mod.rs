use std::io::Error;

pub mod impls;

pub struct ByteBuffer {
    read_idx: usize,
    buf: Vec<u8>,
}

impl ByteBuffer {
    pub fn new(buf: &[u8]) -> ByteBuffer {
        ByteBuffer {
            read_idx: 0,
            buf: Vec::from(buf),
        }
    }

    pub fn skip(&mut self, amount: usize) {
        self.read_idx += amount;
    }

    pub fn zero_to(&mut self, len: usize) {
        self.buf.reserve(len.saturating_sub(self.buf.len()));
        while self.buf.len() < len {
            self.buf.push(0);
        }
    }

    pub fn empty() -> ByteBuffer {
        ByteBuffer {
            read_idx: 0,
            buf: Vec::new(),
        }
    }

    pub fn write<T: WriteBuf + Sized>(&mut self, value: &T) -> Result<(), Error> {
        value.write_buf(self)
    }

    pub fn read<T: ReadBuf>(&mut self) -> Result<T, Error> {
        T::read_buf(self)
    }
}

impl AsRef<[u8]> for ByteBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

pub trait ReadBuf {
    fn read_buf(buf: &mut ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait WriteBuf {
    fn write_buf(&self, buf: &mut ByteBuffer) -> Result<(), Error>
    where
        Self: Sized;
}
