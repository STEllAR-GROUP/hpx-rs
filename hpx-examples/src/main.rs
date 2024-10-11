#![feature(vec_into_raw_parts)]
#![feature(random)]
use core::array::from_fn;
use std::{process, random, ffi::c_char, env};

use hpx_sys as hpx;

fn hpx_main(argc: i32, argv: *mut *mut c_char) -> i32 {

    let numbers: &[i32; 16384] = &from_fn(|_| random::random::<i32>());
    let list: &mut Vec<i32> = &mut Vec::<i32>::from(numbers);
    println!("{:#?}", list);
    // Sort the array in parallel.
    hpx::hpx_sort_comp(list, |a, b| a < b);
    println!("{:#?}", list);
    hpx::finalize();
    return 0;
}
fn main() {
    let args = env::args();
    let (argc, argv) = hpx::create_c_args(&args
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
    );
    unsafe {
    process::exit(hpx::init(hpx_main, argc, argv
            .into_raw_parts()
            .0));
    }
}
