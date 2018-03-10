
fn main() {
    let t: usize = read();
    let _: usize = read();
    let veca: Vec<usize> = read_vec();
    let m: usize = read();
    let vecb: Vec<usize> = read_vec();

    let mut buyer = 0;
    for a in veca {
        if buyer == m {
            break;
        }
        if a + t >= vecb[buyer] && a <= vecb[buyer] {
            buyer += 1;
        }
    }

    println!("{}", if buyer == m { "yes" } else { "no" });
}

fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().parse::<T>().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim()
        .split_whitespace()
        .map(|t| t.parse::<T>().ok().unwrap())
        .collect()
}
