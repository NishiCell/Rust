fn read_num() -> u32 {
    use std::io::*;
    let s = stdin();
    let b = s.bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    b.parse().unwrap()
}
fn main() {
    let a = read_num();
    let b = read_num();
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}
