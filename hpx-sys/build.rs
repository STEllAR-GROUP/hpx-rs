#![allow(unused_imports, dead_code)]
use std::{
    env,
    path::{Path, PathBuf},
};

/// Try to use user installed hpx and emit necessary build script instructions.
fn try_hpx_application() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg
        .range_version("1.10.0".."1.12.0")
        .probe("hpx_application")
    {
        Ok(lib) => {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            Ok(lib)
        }
        Err(e) => {
            println!("cargo:warning=failed to probe hpx_application: {e}");
            Err(e)
        }
    }
}

fn try_hpx_component() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg.range_version("1.10.0".."1.12.0").probe("hpx_component") {
        Ok(lib) => {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            Ok(lib)
        }
        Err(e) => {
            println!("cargo:warning=failed to probe hpx_component: {e}");
            Err(e)
        }
    }
}

/// User is required to specify linker files path for hpx
/// because by default Cargo will export `DYLD_LIBRARY_PATH=/opt/local/lib:...`
/// `export DYLD_LIBRARY_PATH=~/stllr/hpx-install/lib:$DYLD_LIBRARY_PATH` and `export
/// LD_LIBRARY_PATH on linux`
fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_rs_path = Path::new(&manifest_dir).join("src").join("lib.rs");
    println!(
        "cargo:warning=Looking for lib.rs at: {}",
        lib_rs_path.display()
    );
    let mut build = cxx_build::bridge(lib_rs_path);
    //let hpx_application = try_hpx_application();
    let hpx_application = match try_hpx_application() {
        Ok(lib) => lib,
        Err(_) => panic!("Failed to find hpx_application using pkg-config"),
    };

    for path in &hpx_application.include_paths {
        build.include(path);
    }

    println!("cargo::rustc-link-lib=hpx_iostreams");
    println!("cargo:rustc-link-lib=dylib=hpx");
    //cxx_build::bridge("src/lib.rs")
    build
        .file("include/wrapper.cpp")
        .std("c++17")
        .compile("hpx-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=include/wrapper.cpp");
}
