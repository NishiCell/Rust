fn read<T: std::str::FromStr>() -> T {
    use std::io::*;
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().unwrap()
}

fn main() {
    let a: u32 = read();
    let b: u32 = read();

    let ans = if a <= 0 && b >= 0 {
        "Zero"
    } else if a > 0 {
        "Positive"
    } else if (b - a) % 2 == 0 {
        "Negative"
    } else {
        "Positive"
    };

    println!("{}", ans);
}
