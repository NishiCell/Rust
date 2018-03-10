use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a: u32 = a.trim().parse::<u32>().unwrap(); //unwrapを入れないと、Some()で帰ってくる。
    
    let mut bc = String::new();
    io::stdin().read_line(&mut bc);
    let mut bc_split = bc.split_whitespace();
    
    let b :&u32 = &bc_split.next().unwrap().parse::<u32>().unwrap();
    let c :&u32 = &bc_split.next().unwrap().parse::<u32>().unwrap(); 

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    println!("{} {}", a+ b+ c, s.trim());
    // print!("{} {}\n", a+ b+ c, s.trim());
}
