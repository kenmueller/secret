#[macro_use]
extern crate magic_crypt;

mod clip;
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

			let message = encrypt(&password, &input::get()?);

			clip::copy(message.clone())?;
			println!("{} (copied to clipboard)", message);
		}
		"d" => {
			let password = password::get_always()?;

			print!("Decrypt message: ");
			stdout().flush()?;

			let message = decrypt(&password, &input::get()?)?;

			clip::copy(message.clone())?;
			println!("{} (copied to clipboard)", message);
		}
		"p" => {
			print!("Set password: ");
			stdout().flush()?;

			let password = input::get()?;

			password::set(&password)?;
			clip::copy(password)?;

			println!("copied password to clipboard");
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
