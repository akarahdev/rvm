use crate::buf::{ReadBuf, WriteBuf};

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl ReadBuf for User {
    fn read_buf(buf: &mut crate::buf::ByteBuffer) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(User {
            name: buf.read()?,
            password: buf.read()?,
        })
    }
}

impl WriteBuf for User {
    fn write_buf(&self, buf: &mut crate::buf::ByteBuffer) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        buf.write(&self.name)?;
        buf.write(&self.password)
    }
}
