fn main() {
    let n = read();
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..n {
        let a = read();
        vec.push(a);
    }
    let max: usize = vec.into_iter().max().unwrap() as usize;
    vec.swap_remove(max);
    println!("{:?}", vec);
}

fn read() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<u32>().ok().unwrap()
}
