fn main() {
    let mut bytes = "abcdefghijklmnopqrstuvwxyz".bytes();
    for c in 0..20 {
        println!("{:?}", bytes.next());
    }
}
