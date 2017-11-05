fn main() {
    let guess: u64 = "100000000000000".parse().expect("Stupid! Type a number!");
    println!("{}", guess);

    let t = true;
    println!("{}", t);
    let f: bool = false;
    println!("{}", f);

    let tup: (i32, f64, u8) = (500, 6.4, 255);
    let (x, y, z) = tup;
    println!("{}", y);
    println!("{}", x);
    println!("{}", z);
}