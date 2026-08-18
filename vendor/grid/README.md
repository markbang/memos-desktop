# Grid

This vendored `0.18.0` source is kept API-compatible with the version pinned by
GPUI's taffy dependency. It backports the upstream grid dimension-overflow
checks from commit `be213bd3528727148bef2d523c89e95d1fd9c072` until GPUI can
move to a dependency version with the fix. The original MIT license is included
in `LICENSE`.


[![docs](https://docs.rs/grid/badge.svg)](https://docs.rs/grid)
[![crates.io](https://badgen.net/crates/d/grid)](https://crates.io/crates/grid)
[![build status](https://github.com/becheran/grid/actions/workflows/rust.yml/badge.svg)](https://github.com/becheran/grid/actions)
[![license](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Data structure grid for rust. Provide a two dimensional data structure for rust that is easy to use and fast.
Most of the functionality provided by the [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) type for one dimensional vectors
is implemented for two dimensions in this crate.

To use *grid* with *no_std* import the library such as:

``` toml
grid = { version = "*", default-features = false }
```

- [documentation](https://docs.rs/grid/)
- [library on crates.io](https://crates.io/crates/grid)
