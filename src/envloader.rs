pub fn fetch_env () -> u8 {
	if let Ok(e) = std::env::var("RUST_ENV") {
		if e == "production" || e == "prod" {
			0
		} else {
			3
		}
	} else {
		4
	}
}
