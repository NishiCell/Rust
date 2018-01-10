use std::io;

fn main(){
    let mut a = String::new();
    let (b, c): (u32, u32) = String::new();
    let mut s = String::new();

    io::read_line(&mut a);

    println!("{} {}", a+b+c, s);
}
