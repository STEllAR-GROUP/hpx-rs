use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hpx_sys::{copy_vector, create_c_args, ffi};
use std::os::raw::c_char;

fn benchmark_hpx_copy(c: &mut Criterion) {
    c.bench_function("hpx::copy rust benchmark for 1M elements", |b| {
        b.iter(|| {
            let (argc, mut argv) = create_c_args(&["benchmark_hpx_copy"]);

            let hpx_main = |_argc: i32, _argv: *mut *mut c_char| -> i32 {
                let src = vec![1; 1_000_000];
                let _result = copy_vector(black_box(&src));
                ffi::finalize()
            };

            unsafe {
                let _result = ffi::init(hpx_main, argc, argv.as_mut_ptr());
            }
        })
    });
}

criterion_group!(benches, benchmark_hpx_copy);
criterion_main!(benches);
