# knil

Log depending on `RUST_ENV`

## Installation & Usage

```toml
log = "0.4"
knil = "0.1"
```

```rs
#[macro_use]
extern crate log;
extern crate knil;

fn main () {
	// if RUST_ENV isn't set, it'll resort to the default
	// the DEFAULT verbose level is set at TRACE
  knil::construct(None);

  info!("Hello, World!")
}
```

## Using with dotenv 

```toml
[dependencies]
log = "0.4"

[dependencies.knil]
version = "0.1"
features = ["env-loader"]
```

```rs
#[macro_use]
extern crate log;
extern crate knil;

fn main () {
	// the second parameter is the .env file path
	// if none is set, it'll look for `.env` in the root of the project
	knil::construct(4, None);

	info!("Hello, World!")
}
```
