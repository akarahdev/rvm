use std::io::Error;

use buf::{ByteBuffer, ReadBuf};
use os::image::Image;

pub mod buf;
pub mod os;
pub mod terminal;

const MAGIC_NUMBER: [u8; 8] = [114, 118, 109, 73, 109, 97, 103, 101];

fn main() -> Result<(), Error> {
    match std::fs::read("./image.rvm") {
        Ok(image) => {
            let mut image = ByteBuffer::new(&image);
            image.skip(MAGIC_NUMBER.len());
            let image = Image::read_buf(&mut image)?;
            println!("{:?}", image);
            Ok(())
        }
        Err(_) => {
            let image = os::setup::setup_image()?;
            let mut buf = ByteBuffer::empty();
            buf.write(&MAGIC_NUMBER)?;
            buf.write(&image)?;

            std::fs::write("./image.rvm", buf)?;
            Ok(())
        }
    }
}
