use crate::clip;
use crate::crypt;
use crate::file;
use crate::input;
use home::home_dir;
use std::io::{self, stdout, BufReader, Read, Write};

const NAME: &str = ".secret";

fn get_path() -> io::Result<String> {
    home_dir()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
        .and_then(|home| {
            home.join(NAME)
                .to_str()
                .and_then(|path| Some(String::from(path)))
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid home directory path",
                ))
        })
}

pub fn get() -> io::Result<Option<String>> {
    let path = get_path()?;

    let file = match file::open(&path)? {
        Some(file) => file,
        None => return Ok(None),
    };

    let mut reader = BufReader::new(file);
    let mut password = String::new();

    reader.read_to_string(&mut password)?;
    let password = crypt::decrypt(&path, &password)?;

    if password.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Password is empty",
        ));
    }

    Ok(Some(password))
}

pub fn get_always() -> io::Result<String> {
    get()?.ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Password not found",
    ))
}

pub fn set(password: &String) -> io::Result<()> {
    if password.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Your password cannot be empty",
        ));
    }

    let path = get_path()?;
    let password = crypt::encrypt(&path, password);

    file::write(&path, &password)
}

pub fn exists() -> io::Result<bool> {
    match get()? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn assert_exists() -> io::Result<()> {
    if exists()? {
        return Ok(());
    }

    print!("Password: ");
    stdout().flush()?;

    let password = match input::get() {
        Ok(password) => password,
        Err(error) => {
            println!("{}", error);
            return assert_exists();
        }
    };

    if let Err(error) = set(&password) {
        println!("{}", error);
        return assert_exists();
    }

    if let Ok(_) = clip::copy(password) {
        println!("copied password to clipboard");
    }

    Ok(())
}
