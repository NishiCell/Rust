use std::io::*;

fn main() {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace());
    //.take_while(|c| !c.is_whitespace());
    for ii in s {
        println!("{:?}", ii);
    }
}
fn main() {
    let a = [0, -1i32, 1];

    let mut iter = a.into_iter().skip_while(|x| x.is_negative());

    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());
}
