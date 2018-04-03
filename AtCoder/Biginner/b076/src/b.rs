fn main() {
    let n = read();
    let k = read();
    let mut ans = 1;
    for c in 0..n {
        if ans < k {
            ans += ans;
        } else {
            ans += k;
        }
    }
    println!("{}", ans);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}
