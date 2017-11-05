fn main() {
    let s1 = String::from("hello");

    let len = calcu_leng(&s1);

    println!("The length of '{}' is {}", s1, len);

    ideal_world();
}

fn calcu_leng (s: &String) -> usize{
    s.len()
}

fn ideal_world (){
    let mut s = String::from("Ideal");

    s.push_str(",world");

    println!("{}", s);
}