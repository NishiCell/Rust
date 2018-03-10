fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let mut a = 0;
    for _i in s.trim().chars() {
        if a % 2 == 0 {
            print!("{}", _i);
        }
        a += 1;
    }
    println!("");
}
// 文字列出ないから不正解
