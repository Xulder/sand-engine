use rand::random;

pub fn rand_int(n: i32) -> i32 {
    (random::<f64>() * f64::from(n)) as i32
}

pub fn rand_dir() -> i32 {
    let i = rand_int(1000);
    (i % 3) - 1
}

pub fn rand_dir2() -> i32 {
    let i = rand_int(1000);
    if (i % 2) == 0 {
        -1
    } else {
        1
    }
}