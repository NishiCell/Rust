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
    let a = vec![5, 6, 4, 2, 1, 3];
    let (ans, count) = selection_sort(n, a);
    println!("{:?} \n {}", ans, count);
}

fn selection_sort(n: usize, mut a: Vec<u8>) -> (Vec<u8>, u8) {
    let mut count = 0;
    for c in 0..n - 1 {
        let mut minj = c;
        for j in c..n {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        if c != minj {
            a.swap(c, minj);
            count += 1;
        }
    }
    (a, count)
}
