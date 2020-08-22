use log::{Level, LevelFilter, Metadata, Record};
#[cfg(feature = "timestamp")]
use chrono::Local;
#[cfg(feature = "colors")]
use colored::*;

pub type LogResult<T> = Result<T, log::SetLoggerError>;

pub struct Knil(pub LevelFilter);

impl log::Log for Knil {
	fn enabled(&self, m: &Metadata) -> bool {
		self.0 <= m.level()
	}

	fn log(&self, r: &Record) {
		if self.enabled(r.metadata()) {
			#[cfg(feature = "colors")]
			let lvl_str = match r.level() {
				Level::Info => r.level().to_string().cyan(),
				Level::Warn => r.level().to_string().yellow(),
				Level::Error => r.level().to_string().red(),
				Level::Debug => r.level().to_string().purple(),
				_ => r.level().to_string().normal()
			};
			
			#[cfg(not(feature = "colors"))]
			let lvl_str = r.level().to_string();

			let target = if !r.target().is_empty() {
				r.target()
			} else {
				r.module_path().unwrap_or_default()
			};

			#[cfg(feature = "timestamp")]
			println!(
				"[{}] ({}) \"{}\" -> {}",
				Local::now().format("%H:%M:%S"),
				lvl_str,
				target,
				r.args(),
			)
		}
	}

	fn flush(&self) {}
}

impl Knil {
	pub fn new(lvl: u8) -> Self {
		Self(mtf(lvl))
	}
}

fn mtf(lvl: u8) -> log::LevelFilter {
	match lvl {
		0 => log::LevelFilter::Info,
		1 => log::LevelFilter::Warn,
		2 => log::LevelFilter::Error,
		3 => log::LevelFilter::Debug,
		_ => log::LevelFilter::Trace,
	}
}
