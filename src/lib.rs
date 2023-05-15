mod env;
mod logger;

#[cfg(feature = "loadenv")]

/// Using it with dotenv~
///
///```rust
///init("path/to/.env").unwrap();
///
///log::info("Hello, World!")
///```

pub fn init(p: Option<&str>) -> Result<(), log::SetLoggerError> {
	let level = env::read_env(p);

	let knil = Box::new(logger::Knil::new(level));

	log::set_boxed_logger(knil)?;

	let max_lvl = logger::map_to_level(level);

	log::set_max_level(max_lvl);

	Ok(())
}

#[cfg(not(feature = "loadenv"))]

/// Getting started with `Knil`!
/// 
///```rust
///knil::init().unwrap();
///
///log::info!("Hello, World!")
///```

pub fn init() -> Result<(), log::SetLoggerError> {
	let level = env::read_env().expect("cannot read env.");

	let knil = Box::new(logger::Knil::new(level));

	log::set_boxed_logger(knil)?;

	let max_lvl = logger::map_to_level(level);

	log::set_max_level(max_lvl);

	Ok(())
}

