// Accepted
fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<usize>().ok().unwrap()
}
fn read_vec() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| c.parse::<u32>().ok().unwrap())
        .collect::<Vec<u32>>()
}
fn main() {
    let n = read();
    let mut l = read_vec();
    l.sort();
    let mut ans = 0;
    for c in 0..n {
        ans += l[2 * c];
    }
    println!("{}", ans);
}
