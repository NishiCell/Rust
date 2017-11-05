fn main() {
    let n = -15;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0{
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
    if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`
        10 * n
    } else {
        println!(", and is a big number, reduce by two");

        // This expression must return an `i32` as well
        n / 2
        // Todo Try suppressing this expression with a semicolon
    };
    // Dont forget to put a semicolon here! All `let` bindings need it

    println!("{} -> {}", n, big_n);
}