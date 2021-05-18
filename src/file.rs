use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn on_error(error: io::Error) -> io::Result<Option<File>> {
	match error.raw_os_error() {
		Some(code) => {
			if code == 2 {
				Ok(None)
			} else {
				Err(error)
			}
		}
		None => Err(error),
	}
}

pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Option<File>> {
	match File::open(path) {
		Ok(file) => Ok(Some(file)),
		Err(error) => on_error(error),
	}
}

pub fn write<P: AsRef<Path>>(path: P, data: String) -> io::Result<()> {
	let mut file = File::create(path)?;
	file.write_all(data.as_bytes())
}
