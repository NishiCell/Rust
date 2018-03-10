//Accept
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    println!("{}", if s.trim() == "a" { "-1" } else { "a" });
}
