use std::io::Error;

use buf::{ByteBuffer, ReadBuf};
use os::{
    file::{FsEntry, FsRoot},
    image::Image,
    user::User,
};

pub mod buf;
pub mod os;

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
            let image = Image {
                disk_space: 2_u64.pow(20),
                hostname: "somepc".to_string(),
                users: Vec::from([User {
                    name: "akarahdev".to_string(),
                    password: "secretPassword".to_string(),
                }]),
                root_file: FsRoot::new(&[FsEntry::directory("home", &[FsEntry::directory(
                    "akarahdev",
                    &[FsEntry::file("my_file", [2, 4, 5, 3, 1])],
                )])]),
            };
            let mut buf = ByteBuffer::empty();
            buf.write(&MAGIC_NUMBER)?;
            buf.write(&image)?;

            std::fs::write("./image.rvm", buf)?;
            Ok(())
        }
    }
}
