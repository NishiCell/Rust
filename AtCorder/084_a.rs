use std::io;

fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num);
    let x = 48 - num.trim().parse::<u32>().unwrap();
    println!("{}", x);
}
