fn main() {
    let r = read();
    let g = read();
    println!("{}", 2 * g - r);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}
