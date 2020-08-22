#[cfg(feature = "timestamp")]
extern crate chrono;
#[cfg(feature = "colored")]
extern crate colored;
#[cfg(feature = "dotenv")]
extern crate dotenv;
extern crate log;

mod modules;
mod envloader;

pub use modules::{Knil, LogResult};

#[cfg(feature = "dotenv")]
pub fn construct(
	verbose: Option<u8>,
	env_path: Option<&Path>
) -> LogResult<()> {
	let logger = match verbose {
		Some (v) => Knill::new(v),
		None => {
			envloader::load(env_path);
			Knil::new(envloader::fetch_env())
		}
	};

	log::set_max_level(logger.0);
	log::set_boxed_logger(Box::new(logger))
}

#[cfg(not(feature = "dotenv"))]
pub fn construct(verbose: Option<u8>) -> LogResult<()> {
	let logger = match verbose {
		Some (v) => Knil::new(v),
		None => Knil::new(envloader::fetch_env())
	};

	log::set_max_level(logger.0);
	log::set_boxed_logger(Box::new(logger))
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {			
		assert_eq!(2 + 2, 4);
	}
}
