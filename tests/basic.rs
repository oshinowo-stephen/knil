#[macro_use]
extern crate log;
extern crate knil;

#[cfg(feature = "envload")]
use std::path::Path;

#[test]
fn log_basic () {
	#[cfg(feature = "envload")]
	knil::construct(Some(Path::new("test.env"))).ok();

	#[cfg(not(feature = "envload"))]
	knil::construct().ok();

	info!("Print for loader...");
	warn!("Print for loader...");
	error!("Print for loader...");
	debug!("Print for both...");
	trace!("Print for both...")
}
