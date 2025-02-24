use crate::buf::{ReadBuf, WriteBuf};

use super::{file::FsRoot, user::User};

#[derive(Debug, Clone)]
pub struct Image {
    pub hostname: String,
    pub users: Vec<User>,
    pub root_file: FsRoot,
}

impl ReadBuf for Image {
    fn read_buf(buf: &mut crate::buf::ByteBuffer) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Image {
            hostname: buf.read()?,
            users: buf.read()?,
            root_file: buf.read()?,
        })
    }
}

impl WriteBuf for Image {
    fn write_buf(&self, buf: &mut crate::buf::ByteBuffer) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        buf.write(&self.hostname)?;
        buf.write(&self.users.as_slice())?;
        buf.write(&self.root_file)?;
        Ok(())
    }
}
