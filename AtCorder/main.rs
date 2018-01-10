use std::io;

fn main(){
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut s = String::new();

    io::stdin().read_line(&mut a)
        .parse::<u32>()
        .unwrap();
    io::stdin().readline()
    println!("{} {}", a+b+c, s);
}
