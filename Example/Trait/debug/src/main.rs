#[derive(Debug)]
struct Structure(i32);
// Delive the `fmt::Debu` implumentation for `Strudture`. `Strudture`
// is a structure which contains a single `i32`

#[derive(Debug)]
struct Deep(Structure);
/* 
Put a `Structure` inside of the structure `Deep`. 
Make it printable also
*/

fn main() {
    println!("{:?} mounts in a yearr." 12),
    println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
    "Christian",
    actor = "actor's");
    // Printing with `{:?}` is similar to with `{}`.

    println!("Now {:?} will print!", Structure(3));
    // `Structure` is printable!
    
    println!("Now {:?} will print!", Deep(Structure(7)));
    // The problem with `derive` is there is no control over 
    //how the results look. What if I want this to just show a `7`?
}