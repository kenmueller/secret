use clipboard::{ClipboardContext, ClipboardProvider};
use std::io;

fn get_context() -> io::Result<ClipboardContext> {
	ClipboardContext::new().or(Err(io::Error::new(
		io::ErrorKind::Other,
		"Could not open the clipboard",
	)))
}

pub fn copy(data: String) -> io::Result<()> {
	get_context()?.set_contents(data).or(Err(io::Error::new(
		io::ErrorKind::Other,
		"Could not copy to the clipboard",
	)))
}
