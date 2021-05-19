pub fn encrypt(password: String, data: String) -> String {
	format!("{}|{}", password, data)
}

pub fn decrypt(password: String, data: String) -> String {
	format!("{}|{}", password, data)
}
