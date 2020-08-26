#[macro_use]
extern crate log;
extern crate knil;

#[test]
fn log_basic () {
	#[cfg(feature = "env-loader")]
	knil::construct(None, None).ok();

	#[cfg(not(feature = "env-loader"))]
	knil::construct(None).ok();

	info!("Hello, World!");
}
