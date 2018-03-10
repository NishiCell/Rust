fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut vec: Vec<u32> = s.trim().split_whitespace().collect();
    //let mut (tate, yoko) = (0, 0)
    let mut tate = 0;
    let mut yoko = 0;
    for i in vec.iter() {
        let iter_now = vec[i];
        match iter_now {
            "N" => tate += 1,
            "W" => yoko += 1,
            "S" => tate -= 1,
            "E" => yoko -= 1,
        }
    }
    println!("{}", if tate == 0 && yoko == 0 { "Yes" } else { "No" });
}
