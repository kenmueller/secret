use std::io::{self, stdin};

pub fn get() -> io::Result<String> {
    let mut value = String::new();
    stdin().read_line(&mut value)?;

    Ok(String::from(value.trim()))
}
