#[cfg(feature = "dotenv")]
pub fn load(path: Option<&Path>) {
	if let Some(p) = path {
		if let Err(error) = dotenv::dotenv(p) {
			warn!("unable to load from that path")
		}
	} else {
		if let Err(error) = dotenv::dotenv() {
				warn!("unable to load from .env")
		}
	}
}

pub fn fetch_env () -> u8 {
	if let Ok(e) = std::env::var("RUST_ENV") {
		if e != "production" {
			4
		} else {
			1
		}
	} else {
		5
	}
}

