use std::io;
use std::env;

#[cfg(not(feature = "loadenv"))]
pub fn read_env () -> io::Result<usize> {
	let environment = env::var("KNIL_ENV")
		.unwrap_or("development".to_string());

	match environment.parse::<usize>() {
		Ok(env_lvl) => Ok(env_lvl),
		Err(_) => match environment.to_lowercase().as_str() {
			"minimum" | "min" => Ok(0 as usize),
			"maximum" | "max" => Ok(4 as usize),
			"development" | "dev" => Ok(2 as usize),
			"production" | "prod" => Ok(1 as usize),
			_ => Err(io::Error::new(io::ErrorKind::Other, "Invalid environment level."))
		}
	}
}

#[cfg(feature = "loadenv")]
pub fn read_env (path: Option<&str>) -> io::Result<usize> {
	if path.is_none() {
		dotenv::dotenv().ok();
	} else {
		dotenv::from_path(path.unwrap())
			.expect("cannot find \".dotenv\" file.");	
	}

	let environment = env::var("KNIL_ENV")
		.unwrap_or("development".to_string());

	match environment.parse::<usize>() {
		Ok(env_lvl) => Ok(env_lvl),
		Err(_) => match environment.to_lowercase().as_str() {
			"minimum" | "min" => Ok(0 as usize),
			"maximum" | "max" => Ok(4 as usize),
			"development" | "dev" => Ok(2 as usize),
			"production" | "prod" => Ok(1 as usize),
			_ => Err(io::Error::new(io::ErrorKind::Other, "Invalid environment level."))
		}
	}
}
