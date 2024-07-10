# hpx-rs
Rust bindings to HPX, C++ Standard Library for Concurrency and
Parallelism.

## Rust version requirements
hpx-rs works with stable Rust.

## Version of hpx
Currently this library requires hpx-1.10(or newer). The user is required
to pre-install hpx and set the pkg-config to point to "*.pc" files of
hpx. The hpx-sys crate will then link to the hpx.
