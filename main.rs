fn main() {
    let x: Result<u32, &str> = Err("strint");
    println!("{}", x.unwrap());
    //    let xx: Result<u32, &str> = Ok("String").unwrap();

    //    let y: Result<u32, &str> = Err(2).unwrap();
    //   let yy: Result<u32, &str> = Err("Strint").unwrap();
}
