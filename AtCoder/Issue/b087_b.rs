// fn main() {
//     let a: u32 = read();
//     let b: u32 = read();
//     let c: u32 = read();
//     let x: u32 = read();
//     let mut count: u32 = 1;
//     count += fifty_yen(x);
//     count += hundred_yen(x);
//     println!("{}", count);
// }
//
fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().ok().unwrap()
}
//
// fn fifty_yen(x: u32) -> u32 {
//     let a = &x / 500;
//     let bx = x % 500;
//     let ba = bx / 100;
//     a * ba
// }
//
// fn hundred_yen(x: u32) -> u32 {
//     let b = x / 100;
//     b
//
// }
use std::cmp::min;

fn main() {
    let mut a: u32 = read();
    let mut b: u32 = read();
    let c: u32 = read();
    let x: u32 = read();
    let mut count: u32 = 0;
    let mut fifty = min(x / 500, a);
    while a >= 0 {
        let mut ten = min((x - a * 500) / 100, b);
        while b >= 0 {
            let one = (x - a * 500 - b * 100) / 50;
            if one > c {
                break;
            }
            b -= 1;
            count += 1;
        }
        a -= 1;
    }
    println!("{}", count);
}
