fn main(){
    let wan = "理想郷";
    find(wan);

    let tuo = "Ideal World";
    find(tuo);

    println!("{}", &wan[0..3]);
    println!("{}", &tuo[0..5]);
}

fn find(wan :&str){

    for b in wan.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in wan.chars() {
        print!("{}, ", c);
    }

    println!("");
}
