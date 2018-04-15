fn main() {
    let s = String::from("111");
    let p: u32 = s.parse().unwrap();
    println!("{}", p);
}
