fn main() {
    // let mut (x, y) = (String::new(), String::new());
    let mut x = String::new();
    let mut y = String::new();
    let n = read().parse::<u32>();

    for _i in n {
        let x_i: String = read();
        let y_i: String = read();
        mut x.push_str(&x_i);
        mut y.push_str(&y_i);
    }
    println!("{    }{    }", x, y);
}

fn read() -> String {
    let mut i = String::new();
    std::io::stdin().read_line(&mut i).unwrap();
    i
}
