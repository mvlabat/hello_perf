pub fn iter_step_1(mut arr: Vec<i32>) {
    for e in arr.iter_mut() {
        *e *= 3;
    }
}

pub fn iter_step_16(mut arr: Vec<i32>) {
    for e in arr.iter_mut().step_by(16) {
        *e *= 3;
    }
}

pub fn iter_step_64(mut arr: Vec<i32>) {
    for e in arr.iter_mut().step_by(64) {
        *e *= 3;
    }
}

pub fn non_instruction_parallel(mut arr: [i32; 2]) -> (i32, i32) {
    for _ in 0..256 * 1024 * 1024 {
        arr[0] = arr[0].wrapping_add(1);
        arr[0] = arr[0].wrapping_add(1);
    }
    (arr[0], arr[1])
}

pub fn instruction_parallel(mut arr: [i32; 2]) -> (i32, i32) {
    for _ in 0..256 * 1024 * 1024 {
        arr[0] = arr[0].wrapping_add(1);
        arr[1] = arr[1].wrapping_add(1);
    }
    (arr[0], arr[1])
}
