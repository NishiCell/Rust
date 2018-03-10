fn main() {
    let n: u32 = read();
    let mut l: String = read();
    let l = l.split(' ')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    l.sort();
    let mut some = 0;

    for _i in 0..n {
        if l[_i] % 2 == 1 {
            some += 1;
        }
    }

    println!("{}", some);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<T>().ok().unwrap()
}
