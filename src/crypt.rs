use magic_crypt::{MagicCrypt256, MagicCryptError, MagicCryptTrait};
use std::io;

fn get_crypt(password: &String) -> MagicCrypt256 {
	new_magic_crypt!(password, 256)
}

pub fn encrypt(password: &String, data: &String) -> String {
	get_crypt(password).encrypt_str_to_base64(data)
}

pub fn decrypt(password: &String, data: &String) -> io::Result<String> {
	get_crypt(password)
		.decrypt_base64_to_string(data)
		.or_else(|error| match error {
			MagicCryptError::IOError(error) => Err(error),
			MagicCryptError::DecryptError(_) => Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"This message isn't meant for you!",
			)),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"Invalid message",
			)),
		})
}
