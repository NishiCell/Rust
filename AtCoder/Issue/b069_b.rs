fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let ch = s.chars().collect::<Vec<char>>();
    let leng = s.len();
    println!("{}{}{}", ch[0], leng - 3, ch[leng - 2]);
}
