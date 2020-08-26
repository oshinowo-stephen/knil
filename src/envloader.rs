#[cfg(feature = "env-loader")]
use std::path::Path;

#[cfg(feature = "env-loader")]
pub fn load(path: Option<&Path>) {
	if let Some(p) = path {
		if let Err(error) = dotenv::from_path(p) {
			eprintln!("unable to load from that path")
		}
	} else {
		if let Err(error) = dotenv::dotenv() {
			eprintln!("unable to load from .env")
		}
	}
}

pub fn fetch_env () -> u8 {
	if let Ok(e) = std::env::var("RUST_ENV") {
		if e != "production" {
			3
		} else {
			0
		}
	} else {
		69
	}
}

