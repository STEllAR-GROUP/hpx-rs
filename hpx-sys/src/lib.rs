#![doc(html_root_url = "https://github.com/STEllAR-GROUP/hpx-rs")]
#![allow(bad_style, non_camel_case_types, unused_extern_crates)]
#![allow(dead_code, unused_imports)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("hpx-sys/include/wrapper.h");

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
        fn hpx_copy(src: &Vec<i32>, dest: &mut Vec<i32>);
        fn hpx_copy_n(src: &Vec<i32>, count: usize, dest: &mut Vec<i32>);
        fn hpx_copy_if(src: &Vec<i32>, dest: &mut Vec<i32>, pred: fn(i32) -> bool);
        fn hpx_count(vec: &Vec<i32>, value: i32) -> i64;
        fn hpx_count_if(vec: &Vec<i32>, pred: fn(i32) -> bool) -> i64;
        fn hpx_ends_with(src: &[i32], dest: &[i32]) -> bool;
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

    fn count(vec: &Vec<i32>, value: i32) -> i64 {
        ffi::hpx_count(vec, value)
    }

    #[test]
    #[serial]
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

    #[test]
    #[serial]
    fn test_hpx_count() {
        let (argc, mut argv) = create_c_args(&["test_hpx_count"]);

        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let vec = vec![1, 2, 3, 2, 4, 2, 5, 2];
            let result = count(&vec, 2);
            assert_eq!(result, 4);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    #[serial]
    fn test_hpx_count_if() {
        let (argc, mut argv) = create_c_args(&["test_hpx_count_if"]);
        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let result_even = ffi::hpx_count_if(&vec, |x| x % 2 == 0);
            assert_eq!(result_even, 5);
            let result_greater_than_5 = ffi::hpx_count_if(&vec, |x| x > 5);
            assert_eq!(result_greater_than_5, 5);
            let is_prime = |n: i32| {
                if n <= 1 {
                    return false;
                }
                for i in 2..=(n as f64).sqrt() as i32 {
                    if n % i == 0 {
                        return false;
                    }
                }
                true
            };
            let result_prime = ffi::hpx_count_if(&vec, is_prime);
            assert_eq!(result_prime, 4);
            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    #[serial]
    fn test_hpx_ends_with() {
        let (argc, mut argv) = create_c_args(&["test_hpx_ends_with"]);

        let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
            let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let vec2 = vec![8, 9, 10];
            let vec3 = vec![7, 8, 9];
            let vec4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let vec5: Vec<i32> = vec![];

            // checking with complete vectors
            assert!(ffi::hpx_ends_with(&vec1, &vec2));
            assert!(!ffi::hpx_ends_with(&vec1, &vec3));
            assert!(ffi::hpx_ends_with(&vec1, &vec4));
            assert!(ffi::hpx_ends_with(&vec1, &vec5));
            assert!(ffi::hpx_ends_with(&vec5, &vec5));

            // checking for vector slices
            assert!(ffi::hpx_ends_with(&vec1[5..], &vec2));
            assert!(ffi::hpx_ends_with(&vec1[..], &vec1[8..]));
            assert!(!ffi::hpx_ends_with(&vec1[..5], &vec2));
            assert!(ffi::hpx_ends_with(&vec1[..5], &vec1[3..5]));

            assert!(ffi::hpx_ends_with(&vec1, &[]));
            assert!(ffi::hpx_ends_with(&[], &[]));

            ffi::finalize()
        };

        unsafe {
            let result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            assert_eq!(result, 0);
        }
    }
}
