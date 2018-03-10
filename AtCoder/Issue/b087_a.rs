fn main() {
    let x = read();
    let a = read();
    let b = read();
    println!("{}", (x - a) % b);
}

fn read() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<u32>().unwrap()
}
