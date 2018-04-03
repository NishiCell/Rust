extern crate selection_sort;
use selection_sort::*;

fn main() {
    let n = read();
    let a = read_vec();
    let (ans, count) = selection_sort(n, a);
    println!("{:?} \n {}", ans, count);
}

#[test]
fn test() {
    let n: usize = 6;
    let a = [5, 6, 4, 2, 1, 3];
    let (ans, count) = selection_sort(n, a);
    println!("{:?} \n {}", ans, count);
}

fn selection_sort(n: usize, mut a: [u8; 5]) -> ([u8; 5], u8) {
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
