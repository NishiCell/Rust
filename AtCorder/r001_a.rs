fn main(){
    let n = 9;
    let s = 131142143;
    let (a, b, c, d) = (0, 0, 0, 0);
    for _i in s.chars() {
        if s == 1 {
          a += 1
        } else if s == 2 {
          b += 1
        } else if s == 3 {
          c += 1
        } else if s == 4 {
          d += 1
        }
    }
    println!("{
    }{

    }{

    }{

    }", a, b, c, d)
}
