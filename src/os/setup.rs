use std::io::{Error, Write, stdin, stdout};

use super::{file::FsRoot, image::Image, user::User};

pub fn setup_image() -> Result<Image, Error> {
    let hostname = read_hostname()?;
    let users = make_users()?;
    Ok(Image {
        hostname,
        users,
        root_file: FsRoot::new([]),
    })
}

pub fn read_disk_space() -> Result<u64, Error> {
    let amount: u64 = loop {
        print!("How much disk space would you like to allocate to this RVM image? ");
        stdout().flush()?;
        let mut amount_buf = String::new();
        stdin().read_line(&mut amount_buf)?;
        let mut amount_buf = amount_buf.trim().to_ascii_lowercase();
        let mut multiplier = 1;
        if amount_buf.contains("kb") {
            amount_buf = amount_buf.replace("kb", "");
            multiplier = 1000;
        }
        if amount_buf.contains("mb") {
            amount_buf = amount_buf.replace("mb", "");
            multiplier = 1000000;
        }
        match amount_buf.parse::<u64>() {
            Ok(value) => break value * multiplier,
            Err(_) => {
                println!("Could not read a valid value.");
                continue;
            }
        }
    };
    Ok(amount)
}

pub fn read_hostname() -> Result<String, Error> {
    print!("What would you like to name your computer? ");
    stdout().flush()?;
    let mut name = String::new();
    stdin().read_line(&mut name)?;
    Ok(name)
}

pub fn make_users() -> Result<Vec<User>, Error> {
    let mut users = Vec::new();
    loop {
        print!("Would you like to add another user (y/n)? ");
        stdout().flush()?;

        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        let buf = buf.trim();

        match buf {
            "y" | "Y" => {
                print!("Username: ");
                stdout().flush()?;
                let mut name = String::new();
                stdin().read_line(&mut name)?;
                let name = name.trim();

                print!("Password: ");
                stdout().flush()?;
                let mut pass = String::new();
                stdin().read_line(&mut pass)?;
                let pass = name.trim();

                users.push(User {
                    name: name.into(),
                    password: pass.into(),
                });
            }
            "n" | "N" => return Ok(users),
            _ => {
                println!("Please try again with either `y` or `n`!");
            }
        }
    }
}
