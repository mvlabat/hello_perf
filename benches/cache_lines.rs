use criterion::{criterion_group, criterion_main, Criterion, Fun};

use hello_perf::*;

const BIG_ENOUGH: usize = 240 * 1024;

fn bench_loop_step(c: &mut Criterion) {
    let step_1 = Fun::new("Step 1", |b, i: &Vec<i32>| b.iter(|| iter_step_1(i.clone())));
    let step_16 = Fun::new("Step 16", |b, i: &Vec<i32>| b.iter(|| iter_step_16(i.clone())));
    let step_64 = Fun::new("Step 64", |b, i: &Vec<i32>| b.iter(|| iter_step_64(i.clone())));
    c.bench_functions(
        "Cache lines",
        vec![step_1, step_16, step_64],
        criterion::black_box(vec![0; BIG_ENOUGH]),
    );
}

fn bench_instruction_parallelism(c: &mut Criterion) {
    let non_parallel = Fun::new("Non-parallel", |b, i: &[i32; 2]| b.iter(|| non_instruction_parallel(i.clone())));
    let parallel = Fun::new("Parallel", |b, i: &[i32; 2]| b.iter(|| instruction_parallel(i.clone())));
    c.bench_functions(
        "Instruction-level parallelism",
        vec![non_parallel, parallel],
        criterion::black_box([0; 2]),
    );
}

criterion_group!(benches, bench_loop_step, bench_instruction_parallelism);
criterion_main!(benches);
