mod file;
mod key;

fn main() {
	match key::get() {
		Ok(key) => println!("{}", key.unwrap_or(String::from("Key not found"))),
		Err(error) => println!("{}", error),
	};
}
