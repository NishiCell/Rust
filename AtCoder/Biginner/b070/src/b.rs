fn main() {
    let a = read();
    let b = read();
    let c = read();
    let d = read();

    let mut ans = 0;
    if a < c && b < d {
        ans = b - c;
    } else if a < c && d < b {
        ans = d - c;
    } else if b < d && c < a {
        ans = b - a;
    } else if a > c && b > d {
        ans = d - a;
    } else {
    }
    println!("{}", if ans >= 0 { ans } else { 0 });
}

fn read() -> i32 {
    use std::io::*;
    let s = stdin();
    let b = s.bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    b.parse().ok().unwrap()
}
