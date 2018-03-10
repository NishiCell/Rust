fn getline() -> (u64, u64) {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    let split: Vec<_> = ret.trim().split(' ').collect();
    let a: u64 = split[0].parse().unwrap();
    let k: u64 = split[1].parse().unwrap();
    (a, k)
}

fn main() {
    let (mut a, k) = getline();
    let mut i = 0;
    if k == 0 {
        println!("{}", 2000000000000 - a);
    } else {
        while a < 2000000000000 {
            a = a + 1 + k * a;
            i += 1;
        }
        println!("{}", i);
    }
}
