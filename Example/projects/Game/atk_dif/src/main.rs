fn main() {

    atk_dif(8,6);

    println!("{}", atack_deff());
}

fn atk_dif(x: u32, y: u32){
    let A = x - y / 2;
    println!("{}", A);
}

fn atack_deff() -> u32{
    atack() - deff() / 2
}

fn atack() -> u32{
    3
}

fn deff() -> u32{
    4
}
