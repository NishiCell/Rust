fn round_n(mut x: f32, n: u32) -> f32 {
    for _i in 0..n - 1 {
        x *= 10.0
    }
    x += 0.5;
    let mut x = x as u32;
    let mut x = x as f32;

    for _i in 0..n-1 {
        x *= 0.1
    }

    return x;
}

fn main() {
    assert_eq!(round_n(0.05, 2), 0.1);
}
