mod crypt;
mod file;
mod input;
mod password;

use crypt::{decrypt, encrypt};
use std::io::{self, stdout, Write};

fn did_quit() -> io::Result<bool> {
	print!("(e)ncrypt (d)ecrypt (p)assword (q)uit ");
	stdout().flush()?;

	match input::get()?.as_str() {
		"e" => {
			let password = password::get_always()?;

			print!("Encrypt message: ");
			stdout().flush()?;

			println!("{}", encrypt(password, input::get()?));
		}
		"d" => {
			let password = password::get_always()?;

			print!("Decrypt message: ");
			stdout().flush()?;

			println!("{}", decrypt(password, input::get()?));
		}
		"p" => {
			print!("Set password: ");
			stdout().flush()?;

			password::set(input::get()?)?;
		}
		"q" => return Ok(true),
		_ => {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"Invalid option",
			))
		}
	};

	Ok(false)
}

fn main() -> io::Result<()> {
	password::assert_exists()?;

	loop {
		match did_quit() {
			Ok(quit) => {
				if quit {
					break;
				}
			}
			Err(error) => println!("{}", error),
		}
	}

	Ok(())
}
