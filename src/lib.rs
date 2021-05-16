#[cfg(feature = "stamps")]
extern crate chrono;
#[cfg(feature = "colors")]
extern crate colored;
#[cfg(feature = "loadenv")]
extern crate dotenv;
extern crate log;

mod env;
mod logger;

#[cfg(feature = "loadenv")]

/// Using it with dotenv~
///
///```rust
///init("path/to/.env")?;
///
///info!("Hello, World!")
///```

pub fn init(p: &str) -> Result<(), log::SetLoggerError> {
	let level = env::read_env(if p.len() { None } else { Some(p) });

	let knil = Box::new(logger::Knil::new(level));

	log::set_boxed_logger(knil);
	log::set_max_level(log::LevelFilter::Trace);

	Ok(())
}

#[cfg(not(feature = "loadenv"))]

/// Getting started with `Knil`!
///
///```rust
///knil::init()?;
///
///info!("Hello, World!")
///```

pub fn init() -> Result<(), log::SetLoggerError> {
	let level = env::read_env().expect("cannot read env.");

	let knil = Box::new(logger::Knil::new(level));

	log::set_boxed_logger(knil)?;
	log::set_max_level(log::LevelFilter::Trace);

	Ok(())
}
