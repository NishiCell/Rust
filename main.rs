fn main() {
    let s = read();
}

fn read() -> u32 {
    use std::io::*;
    let s = stdin();
    let b = s.bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    b.parse().unwrap()
}
