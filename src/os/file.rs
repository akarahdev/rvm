use std::io::{Error, ErrorKind};

use crate::buf::{ReadBuf, WriteBuf};

#[derive(Debug, Clone)]
pub struct FsEntry {
    name: String,
    contents: Contents,
}

impl FsEntry {
    pub fn file(name: impl Into<String>, contents: impl Into<Vec<u8>>) -> FsEntry {
        FsEntry {
            name: name.into(),
            contents: Contents::File(contents.into()),
        }
    }

    pub fn directory(name: impl Into<String>, contents: impl Into<Vec<FsEntry>>) -> FsEntry {
        FsEntry {
            name: name.into(),
            contents: Contents::Directory(contents.into()),
        }
    }
}

impl ReadBuf for FsEntry {
    fn read_buf(buf: &mut crate::buf::ByteBuffer) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(FsEntry {
            name: buf.read()?,
            contents: buf.read()?,
        })
    }
}

impl WriteBuf for FsEntry {
    fn write_buf(&self, buf: &mut crate::buf::ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        buf.write(&self.name)?;
        buf.write(&self.contents)
    }
}

#[derive(Debug, Clone)]
pub struct FsRoot {
    entries: Vec<FsEntry>,
}

impl FsRoot {
    pub fn new(entries: impl Into<Vec<FsEntry>>) -> FsRoot {
        FsRoot {
            entries: entries.into(),
        }
    }
}

impl ReadBuf for FsRoot {
    fn read_buf(buf: &mut crate::buf::ByteBuffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(FsRoot {
            entries: buf.read()?,
        })
    }
}

impl WriteBuf for FsRoot {
    fn write_buf(&self, buf: &mut crate::buf::ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        buf.write(&self.entries.as_slice())
    }
}

#[derive(Debug, Clone)]
pub enum Contents {
    File(Vec<u8>),
    Directory(Vec<FsEntry>),
}

impl ReadBuf for Contents {
    fn read_buf(buf: &mut crate::buf::ByteBuffer) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        match buf.read::<u8>()? {
            1 => Ok(Contents::File(buf.read()?)),
            2 => Ok(Contents::Directory(buf.read()?)),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected file type of file (1) or directory (2).",
            )),
        }
    }
}

impl WriteBuf for Contents {
    fn write_buf(&self, buf: &mut crate::buf::ByteBuffer) -> Result<(), Error>
    where
        Self: Sized,
    {
        match self {
            Contents::File(items) => {
                buf.write(&1u8)?;
                buf.write(&items.as_slice())
            }
            Contents::Directory(files) => {
                buf.write(&2u8)?;
                buf.write(&files.as_slice())
            }
        }
    }
}
