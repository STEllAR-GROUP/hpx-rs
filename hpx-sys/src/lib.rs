#![doc(html_root_url = "https://github.com/STEllAR-GROUP/hpx-rs")]
#![allow(bad_style, non_camel_case_types, unused_extern_crates)]
#![allow(dead_code, unused_imports)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("hpx-sys/include/wrapper.h");

        //fn start() -> i32;
        unsafe fn init(
            func: unsafe fn(i32, *mut *mut c_char) -> i32,
            argc: i32,
            argv: *mut *mut c_char,
        ) -> i32;

        fn finalize() -> i32;
        fn finalize_with_timeout(shutdown_timeout: f64, localwait: f64) -> i32;
        fn terminate();
        fn disconnect() -> i32;
        fn disconnect_with_timeout(shutdown_timeout: f64, localwait: f64) -> i32;
        //fn stop() -> i32;
    }
}

// ================================================================================================
// Tests (to be shifted to systests crate within hpx-rs workspace)
// ================================================================================================
#[cfg(test)]
mod tests {
    use super::ffi;
    use serial_test::serial;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::thread;
    use std::time::Duration;

    fn create_c_args(args: &[&str]) -> (i32, Vec<*mut c_char>) {
        let c_args: Vec<CString> = args.iter().map(|s| CString::new(*s).unwrap()).collect();
        let ptrs: Vec<*mut c_char> = c_args.iter().map(|s| s.as_ptr() as *mut c_char).collect();
        (ptrs.len() as i32, ptrs)
    }

    #[test]
    //#[serial]
    fn test_init_finalize() {
        let (argc, mut argv) = create_c_args(&["testing", "arg1", "arg2"]);

        let dummy_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            println!("Dummy fn called");
            // to exit hpx::init you are required to shutdown hpx runtime
            ffi::finalize();
            0
        };

        unsafe {
            let result = ffi::init(dummy_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }
}
