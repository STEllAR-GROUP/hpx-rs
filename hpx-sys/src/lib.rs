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
        fn hpx_copy(src: &Vec<i32>, dest: &mut Vec<i32>);
        fn hpx_copy_n(src: &Vec<i32>, count: usize, dest: &mut Vec<i32>);
        fn hpx_copy_if(src: &Vec<i32>, dest: &mut Vec<i32>, pred: fn(i32) -> bool);
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

    fn copy_vector(src: &Vec<i32>) -> Vec<i32> {
        let mut dest = vec![0; src.len()];
        ffi::hpx_copy(src, &mut dest);
        dest
    }

    fn copy_vector_range(src: &Vec<i32>, start: usize, end: usize) -> Vec<i32> {
        let slice = &src[start..end];
        let mut dest = vec![0; slice.len()];
        ffi::hpx_copy(&slice.to_vec(), &mut dest);
        dest
    }

    fn copy_n(src: &[i32], count: usize) -> Vec<i32> {
        let mut dest = Vec::with_capacity(count);
        ffi::hpx_copy_n(&src.to_vec(), count, &mut dest);
        dest
    }

    fn copy_if_positive(src: &Vec<i32>) -> Vec<i32> {
        let mut dest = Vec::new();
        ffi::hpx_copy_if(src, &mut dest, |x| x % 3 == 0);
        dest
    }
    //#[test]
    //fn test_init_finalize() {
    //    let (argc, mut argv) = create_c_args(&["testing", "arg1", "arg2"]);
    //
    //    let dummy_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
    //        println!("Dummy fn called");
    //        // to exit hpx::init you are required to shutdown hpx runtime
    //        ffi::finalize();
    //        0
    //    };
    //
    //    unsafe {
    //        let result = ffi::init(dummy_main, argc, argv.as_mut_ptr());
    //        assert_eq!(result, 0);
    //    }
    //}

    #[test]
    #[serial]
    fn test_hpx_copy() {
        let (argc, mut argv) = create_c_args(&["test_hpx_copy"]);

        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let src = vec![1, 2, 3, 4, 5];
            let result = copy_vector(&src);
            assert_eq!(src, result);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    #[serial]
    fn test_hpx_copy_range() {
        let (argc, mut argv) = create_c_args(&["test_hpx_copy"]);

        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let src = vec![1, 2, 3, 4, 5];
            let result = copy_vector_range(&src, 0, 3);
            assert_eq!(&src[0..3], &result);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    #[serial]
    fn test_copy_n() {
        let (argc, mut argv) = create_c_args(&["test_copy_n"]);

        let test_func = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let src = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let result = copy_n(&src, 5);
            assert_eq!(result, vec![1, 2, 3, 4, 5]);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(test_func, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    #[serial]
    fn test_hpx_copy_if() {
        let (argc, mut argv) = create_c_args(&["test_hpx_copy_if"]);

        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let src = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
            let result = copy_if_positive(&src);
            assert_eq!(result, vec![0, 3, 6, 9, 12]);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }
}
