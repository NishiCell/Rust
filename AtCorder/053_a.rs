use std::io;

fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num);
    let num :u32 
        = num.trim().parse::<u32>().unwrap();
    if num < 1200 && num >= 1{
        println!("ABC");
    }else if num >= 1200 && num <= 3000 {
        println!("ARC");
    };
}
