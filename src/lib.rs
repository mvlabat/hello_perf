use std::{
    thread,
    sync::Arc,
};

pub fn iter_step_one(mut arr: Vec<i32>) -> Vec<i32> {
    for e in arr.iter_mut() {
        *e *= 3;
    }
    arr
}

pub fn iter_step(mut arr: Vec<i32>, iter_step: usize) -> Vec<i32> {
    for e in arr.iter_mut().step_by(iter_step) {
        *e *= 3;
    }
    arr
}

pub fn non_instruction_parallel(mut arr: [i32; 2], a: i32, b: i32, c: usize) -> (i32, i32) {
    for _ in 0..c {
        arr[0] = arr[0].wrapping_mul(a);
        arr[0] = arr[0].wrapping_div(b);
    }
    (arr[0], arr[1])
}

pub fn instruction_parallel(mut arr: [i32; 2], a: i32, b: i32, c: usize) -> (i32, i32) {
    for _ in 0..c {
        arr[0] = arr[0].wrapping_mul(a);
        arr[1] = arr[1].wrapping_div(b);
    }
    (arr[0], arr[1])
}

pub fn cache_line_sharing(arr: [i32; 128], pos: usize) -> (i32, i32) {
    let arr = Arc::new(arr);
    let handles: Vec<_> = (0..4).map(|thread_number| {
        let arr = arr.clone();
        let pos = thread_number * pos;
        thread::spawn(move || unsafe {
            let p = (arr.as_ptr() as *mut i32).offset(pos as isize);
            for _ in 0..1_000_000 {
                *p = *p.wrapping_add(3);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    (arr[0], arr[1])
}
