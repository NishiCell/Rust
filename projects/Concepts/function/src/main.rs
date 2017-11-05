fn main() {
    println!("Hello, world!");

    sub_function();

    num_function(255, -4);

    let z = five();
    println!("Five!{}", z);

    let p = plus_one(5);
    println!("Six!{}", p);
}

fn sub_function(){
    println!("My Ideal World!!");
}

fn num_function(x: u8, y:i32){
    println!("{}", x);
    println!("{}", y);
}

fn five() -> i32{
    5;
}

fn plus_one(p: i32) -> i32{
    p + 1;
}