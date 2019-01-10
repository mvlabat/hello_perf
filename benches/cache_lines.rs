use criterion::{criterion_group, criterion_main, Criterion, Fun};

use hello_perf::*;

const BIG_ENOUGH: usize = 240 * 1024;

fn bench_loop_step(c: &mut Criterion) {
    let step_1 = Fun::new("Step 1", |b, i: &Vec<i32>| b.iter(|| {
        iter_step_one(i.clone());
    }));
    let step_16 = Fun::new("Step 16", |b, i: &Vec<i32>| b.iter(|| {
        iter_step(i.clone(), 16);
    }));
    let step_64 = Fun::new("Step 64", |b, i: &Vec<i32>| b.iter(|| {
        iter_step(i.clone(), 64);
    }));
    c.bench_functions(
        "Cache lines",
        vec![step_1, step_16, step_64],
        criterion::black_box(vec![1; BIG_ENOUGH]),
    );
}

fn bench_instruction_parallelism(c: &mut Criterion) {
    let non_parallel = Fun::new("Non-parallel", |b, i: &([i32; 2], i32, i32, usize)| b.iter(|| {
        non_instruction_parallel(i.0.clone(), i.1, i.2, i.3)
    }));
    let parallel = Fun::new("Parallel", |b, i: &([i32; 2], i32, i32, usize)| b.iter(|| {
        instruction_parallel(i.0.clone(), i.1, i.2, i.3)
    }));
    c.bench_functions(
        "Instruction-level parallelism",
        vec![non_parallel, parallel],
        criterion::black_box(([32768; 2], 3, 3, BIG_ENOUGH)),
    );
}

fn bench_false_cache_line_sharing(c: &mut Criterion) {
    let false_sharing = Fun::new("False cache line sharing", |b, i: &Vec<i32>| b.iter(|| {
        cache_line_sharing(slice_to_arr(&i), 1);
    }));
    let no_false_sharing = Fun::new("No false cache line sharing", |b, i: &Vec<i32>| b.iter(|| {
        cache_line_sharing(slice_to_arr(&i), 16);
    }));
    c.bench_functions(
        "Cache line sharing",
        vec![false_sharing, no_false_sharing],
        criterion::black_box(vec![1; 128]),
    );
}

fn slice_to_arr<T>(slice: &[T]) -> [T; 128]
    where T: Default + Copy,
{
    let mut arr = [T::default(); 128];
    let slice = &slice[..arr.len()];
    arr.copy_from_slice(slice);
    arr
}

criterion_group!(
    benches,
    bench_loop_step,
    bench_instruction_parallelism,
    bench_false_cache_line_sharing,
);
criterion_main!(benches);
