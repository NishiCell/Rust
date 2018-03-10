fn main(){
    let mut iter = "Poi Poy Pon".split_whitespace();

    println!("{}", iter.next().next().unwrap());
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());

}
