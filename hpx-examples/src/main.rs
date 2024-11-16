use core::array::from_fn;
use std::{env, process};
use rand;

fn hpx_main(_: Vec<String>) -> i32 {
    let numbers: &[i32; 16384] = &from_fn(|_| rand::random::<i32>());
    let list: &mut Vec<i32> = &mut Vec::<i32>::from(numbers);
    println!("{:#?}", list);
    // Sort the array in parallel.
    hpx_sys::ffi::hpx_sort_comp(list, |a, b| a < b);
    println!("{:#?}", list);
    hpx_sys::ffi::finalize();
    return 0;
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    process::exit(hpx_sys::init(hpx_main, args));
}
