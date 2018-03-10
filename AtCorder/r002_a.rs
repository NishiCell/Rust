fn main(){
    let x = read();
    if x % 400 == 0 {
        println!("YES");
    } else {
        if x % 4 == 0 {
            if x % 100 == 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        } else {
            println!("NO");
        }
    }
}

fn read () -> u32{
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok();
    num.trim().parse::<u32>().unwrap()
}
