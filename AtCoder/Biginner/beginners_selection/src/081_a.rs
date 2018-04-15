fn main() {
    let n: u32 = read();
    let mut vec: Vec<u32> = Vec::new();
    for c in 0..n {
        vec.push(read())
    }
    let mut kisu_nai = true;
    let mut count = 0;
    let mut sonzai = true;
    while sonzai {
        for c in vec.iter() {
            if c % 2 == 1 {
                kisu_nai = false;
                break;
            }
        }
        if kisu_nai {
            vec.iter().map(|c| c / 2);
        }
    }
    println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
    use std::io::*;
    let s = stdin();
    let b = s.bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    b.parse::<T>().ok().unwrap()
}
