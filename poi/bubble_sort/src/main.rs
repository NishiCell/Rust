fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<T>().ok().unwrap()
}
fn read_vec() -> Vec<u8> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| c.parse().ok().unwrap())
        .collect()
}
fn main() {
    // let n = read();
    // let mut a = read_vec();
    let n: usize = 5;
    let a = [5, 3, 2, 4, 1];
    let (ans, count) = bubble_sort(n, a);
    println!("{:?} \n {}", ans, count);
}

fn bubble_sort(n: usize, mut a: [u8; 5]) -> ([u8; 5], u8) {
    let mut flag = true;
    let mut count = 0;
    while flag {
        let mut c = n - 1;
        flag = false;
        while c > 0 {
            if a[c] < a[c - 1] {
                // let v = a[c];
                // a[c] = a[c - 1];
                // a[c - 1] = v;
                a.swap(c, c - 1);
                flag = true;
                count += 1;
            }
            c -= 1;
        }
    }
    (a, count)
}
