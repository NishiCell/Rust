fn main() {
    let d = read();
    let n: String = read_byte();
    let k: u32 = read_byte();
    let n = n.chars();
    for i in 0..n.len() {
        for j in 0..k {
            if n[i] == d[j] {
                if n[i] == 9 && i != 0 {
                    n[i - 1] += 1;
                    n[i] == 0;
                    break;
                } else {
                    n[i] += 1;
                    break;
                }
            }
        }
    }
    println!("{}", n.to_string());
}

fn read_byte<T: std::str::FromStr>() -> T {
    use std::io::*;
    let s = stdin();
    let b = s.bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    b.parse::<T>().ok().unrap()
}

fn read() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitestace()
        .map(|c| c.parse().ok().unwrap())
        .collect();
}
