# bsdiff-rs

bsdiff-rs is an implementation of the bsdiff algorithm for generating and applying binary patches.

http://www.daemonology.net/bsdiff/

In addition to the raw algorithm, there is support for two container formats.
- BsDiff43  -> https://github.com/mendsley/bsdiff
- JBsDiff40 -> https://github.com/malensek/jbsdiff

## Using

Currently, this project is not in crates.rs. For now, use it by cloning the repo from github. To add it as a dependency, add the following into your cargo.toml:

```toml
[dependencies]
bsdiff-rs = { git = "https://github.com/robot-rover/bsdiff-rs" }
```

## Optional Features

bsdiff-rs also supports using mendsley/bsdiff as a backend and wrapping the C code. To use this rather than the rust backend, use the `c_backend` feature. To build this, you must also clone the submodules for this repo.

## Tests

To run basic unit tests, simply run `cargo test`. However, there are also more complicated integration tests. To use these, first run `./test_setup.sh`. This will build the bsdiff C executables and the jbsdiff jar file which are used in the tests. Then, run `cargo test --features=integration_test`. To run these, you must also clone the submodules for this repo.