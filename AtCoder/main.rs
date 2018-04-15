fn main() {
    let mut bytes = "Ideal World !!".bytes();
    let from = String::from_utf8_lossy(&bytes);
    for c in 0..bytes.len() {
        println!("{:?}", bytes.next());
    }
    println!("{:?}", from);
}
