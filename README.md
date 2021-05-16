# knil

An simple logger based on environment!

## Getting Started!

```toml
# get log and the package from crates

[dependencies]
log = "0.4"
knil = "0.1"
```

```rs
#[macro_use]
extern crate log;
extern crate knil;

fn main () {
	knil::init().expect("failed to build logger~");

	info!("Hello, World!")
}
```

What about the environment thing? Don't worry it's very simple, the environment
soley depends on the: `RUST_ENV`, you can directly set the environment level by
setting the `RUST_ENV` with the corresponding level:

```
error = 0
warn = 1
info = 2
debug = 3
trace = 4
```

There also level presets such as:

```
development = 2
production = 1
max = 4
min = 0
```

By default the level is set to the `development` preset.

## Contribute

Uh, make an Issue or Pull Request if you wanna see something changed or configured.
