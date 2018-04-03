pub fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<usize>().ok().unwrap()
}
pub fn read_vec() -> Vec<u8> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| c.parse().ok().unwrap())
        .collect()
}
