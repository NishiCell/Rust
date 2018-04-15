fn main() {
    let vec = read();
    let n = vec[0];
    let a = vec[1];
    println!("{}", std::cmp::min(a - 1, n - a));
}
fn read() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()
}
