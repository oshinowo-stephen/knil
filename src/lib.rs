#[cfg(feature = "timestamp")]
extern crate chrono;
#[cfg(feature = "colored")]
extern crate colored;
#[cfg(feature = "envload")]
extern crate dotenv;
extern crate log;

mod modules;
mod envloader;

pub use modules::{Knil, LogResult};

#[cfg(feature = "envload")]
use std::path::Path;
use std::env;

#[cfg(feature = "envload")]
pub fn construct (
	path: Option<&Path>
) -> LogResult<()> {
	if let Some (p) = path {
		dotenv::from_path(p);
	}	else {
		dotenv::dotenv().ok();
	}

	let verbose = env::var("KNIL_VERBOSE");

	let logger = match verbose {
		Ok(v) => Knil::new(v.parse::<u8>().unwrap()),
		Err(error) => match error {
			_ => Knil::new(envloader::fetch_env())
		}
	};

	log::set_max_level(logger.0);
	log::set_boxed_logger(Box::new(logger))
}

#[cfg(not(feature = "envload"))]
pub fn construct () -> LogResult<()> {
	let verbose = env::var("KNIL_VERBOSE");

	let logger = match verbose {
		Ok(v) => Knil::new(v.parse::<u8>().unwrap()),
		Err(error) => match error {
			_ => Knil::new(envloader::fetch_env())
		}
	};

	log::set_max_level(logger.0);
	log::set_boxed_logger(Box::new(logger))
}
