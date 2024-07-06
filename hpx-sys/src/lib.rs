//!
#![allow(bad_style, non_camel_case_types, unused_extern_crates)]
#![allow(dead_code)]

#[cxx::bridge]
pub mod ffi {}

#[cfg(test)]
mod tests {
    use super::ffi;
}
