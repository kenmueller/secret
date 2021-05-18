use crate::file;
use home::home_dir;
use std::io::{self, BufReader, Read};
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
	let file = match file::open(get_path()?)? {
		Some(file) => file,
		None => return Ok(None),
	};

	let mut reader = BufReader::new(file);
	let mut key = String::new();

	reader.read_to_string(&mut key)?;

	Ok(Some(key))
}
