fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut iter = s.trim()
        .split_whitespace()
        .map(|c| c.parse::<u32>().ok().unwrap());
    println!(
        "{}",
        (iter.next().unwrap() - 1) * (iter.next().unwrap() - 1)
    );
}
