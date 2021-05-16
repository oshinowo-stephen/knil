use std::env;
use std::io;

fn parse_env(e: &str) -> io::Result<usize> {
	match e.parse::<usize>() {
		Ok(env_lvl) => Ok(env_lvl),
		Err(_) => match e.to_lowercase().as_str() {
			"minimum" | "min" => Ok(0_usize),
			"maximum" | "max" => Ok(4_usize),
			"development" | "dev" => Ok(2_usize),
			"production" | "prod" => Ok(1_usize),
			_ => Err(io::Error::new(
				io::ErrorKind::Other,
				"Invalid environment level.",
			)),
		},
	}
}

#[cfg(not(feature = "loadenv"))]
pub fn read_env() -> io::Result<usize> {
	let environment = env::var("KNIL_ENV").unwrap_or_else(|_| "development".to_string());

	parse_env(&environment)
}

#[cfg(feature = "loadenv")]
pub fn read_env(path: Option<&str>) -> io::Result<usize> {
	if path.is_none() {
		dotenv::dotenv().ok();
	} else {
		dotenv::from_path(path.unwrap()).expect("cannot find \".dotenv\" file.");
	}

	let environment = env::var("KNIL_ENV").unwrap_or("development".to_string());

	parse_env(&environment)
}
