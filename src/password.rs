use crate::file;
use crate::input;
use home::home_dir;
use std::io::{self, stdout, BufReader, Read, Write};
use std::path::PathBuf;

const NAME: &str = ".secret";

fn get_path() -> io::Result<PathBuf> {
    home_dir()
        .and_then(|home| Some(home.join(NAME)))
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
}

pub fn get() -> io::Result<Option<String>> {
    let file = match file::open(&get_path()?)? {
        Some(file) => file,
        None => return Ok(None),
    };

    let mut reader = BufReader::new(file);
    let mut password = String::new();

    reader.read_to_string(&mut password)?;

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

    file::write(&get_path()?, password)
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

    set(&password).or_else(|error| {
        println!("{}", error);
        assert_exists()
    })
}
