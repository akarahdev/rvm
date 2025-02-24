use std::io::Error;

use buf::{ByteBuffer, ReadBuf};
use image::Image;

pub mod buf;
pub mod image;

fn main() -> Result<(), Error> {
    match std::fs::read("./image.rvm") {
        Ok(image) => {
            let mut image = ByteBuffer::new(&image);
            let image = Image::read_rwbuf(&mut image)?;
            println!("{:?}", image);
            Ok(())
        }
        Err(_) => {
            let image = Image {
                hostname: "somepc".to_string(),
            };
            let mut buf = ByteBuffer::empty();
            buf.write(&image)?;

            std::fs::write("./image.rvm", buf)?;
            Ok(())
        }
    }
}
