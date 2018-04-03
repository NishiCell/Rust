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
        flag = false;
        for c in 1..n {
            if a[n - c] < a[n - c - 1] {
                a.swap(n - c, n - c - 1);
                flag = true;
                count += 1;
            }
        }
    }
    (a, count)
}
