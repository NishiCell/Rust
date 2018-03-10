
fn main() {
    let n: usize = read();
    let mut t: Vec<u32> = Vec::new();
    for i in 0..n {
        t.push(read());
    }
    let mut out = t[0];
    for _i in 1..n {
        out = std::cmp::min(out, t[_i]);
    }
    println!("{}", out);
}

fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().ok().unwrap()
}
