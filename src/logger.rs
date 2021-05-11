#[cfg(feature = "colors")]
use colored::*;

fn get_lvl_str (lvl: log::Level) -> String {
	#[cfg(feature = "colors")]
	let lvl_str = match	lvl {
		log::Level::Info =>
			lvl.to_string().cyan(),
		log::Level::Warn =>
			lvl.to_string().yellow(),
		log::Level::Error =>
			lvl.to_string().red(),
		log::Level::Debug =>
			lvl.to_string().purple(),
		log::Level::Trace =>
			lvl.to_string().normal(),
	};
	
	#[cfg(not(feature = "colors"))]
	let lvl_str = lvl;


	lvl_str.to_string()
}

fn map_to_level (int: usize) -> log::LevelFilter {
	match int {
		0 => log::LevelFilter::Error,
		1 => log::LevelFilter::Warn,
		2 => log::LevelFilter::Info,
		3 => log::LevelFilter::Debug,
		_ => log::LevelFilter::Trace,
	}
}

pub struct Knil {

	level: usize

}

impl Knil {
	
	pub fn new (level: usize) -> Self {
		Self { level }
	}

}

impl log::Log for Knil {

	fn flush (&self) {}

	fn log (&self, r: &log::Record) {
		let lvl_str = get_lvl_str(r.level());

		let target = if !r.target().is_empty() {
			r.target()
		} else {
			r.module_path().unwrap_or_default()
		};

		#[cfg(feature = "stamps")]
		use chrono::Local;

		#[cfg(feature = "stamps")]
		println!(
			"[{}] ({}) \"{}\" | {}",
			Local::now().format("%H:%M:%S"),
			lvl_str,
			target,
			r.args(),
		);

		#[cfg(not(feature = "stamps"))]
		println!(
			"({}) \"{}\" | {}",
			lvl_str,
			target,
			r.args(),
		);
	}

	fn enabled(&self, m: &log::Metadata) -> bool {
		m.level() <= map_to_level(self.level) 
	}

}

