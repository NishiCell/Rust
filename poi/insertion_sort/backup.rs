fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<usize>().ok().unwrap()
}
fn read_vec() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| c.parse().ok().unwrap())
        .collect()
}
fn main() {
    let n = read();
    let mut a = read_vec();
    let mut c: usize = 1;
    while c <= n {
        let v = a[c];
        let mut j: i8 = c as i8 - 1;
        while j >= 0 && a[j as usize] > v {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
        }
        a[(j + 1) as usize] = v;
        c += 1;
        println!("{:?}", a);
    }
}
