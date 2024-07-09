//!
//! # HPX bindings for Rust
//! This library contains bindings to [HPX][1], C++ Standard Library for Concurrency and
//! Parallellism. This Library is a work in progress and lacks several bindings, So be warned!
//! It will build the library and link it to HPX. To use this library you need to set
//! PKG_config_PATH to point to hpx "*.pc" files by executing the following in terminal:
//! ```
//! export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:$HPX_LOCATION/lib/pkgconfig
//! ```
//! If on mac you also need to set DYLD path using the following:
//! ```
//! export DYLD_LIBRARY_PATH=$HPX_LOCATION/lib:$DYLD_LIBRARY_PATH
//! ```
//! [1]: https://hpx.stellar-group.org/
