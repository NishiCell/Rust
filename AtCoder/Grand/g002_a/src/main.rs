// Accepted
// 実験課題：
// read_line()との速度の比較
fn read() -> f64 {
    use std::io::*;
    let stdin = stdin();
    let s: String = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<f64>().ok().unwrap()
}
fn main() {
    let a = read();
    let b = read();

    if a > 0.0 && b > 0.0 {
        println!("Positive");
    } else if a <= 0.0 && b >= 0.0 {
        println!("Zero");
    } else if (a < 0.0 && b < 0.0) && (b - a) % 2.0 == 1.0 {
        println!("Positive");
    } else {
        println!("Negative");
    }
}
