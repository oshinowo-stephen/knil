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