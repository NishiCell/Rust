fn main() {
    use std::collections::VecDeque;

    let mut vec = VecDeque::new();
    vec.push_back(3);
    vec.push_back(4);
    vec.push_back(5);
    vec.push_back(6);
    vec.push_back(7);
    vec.push_back(8);

    for c in vec {
        println!("{}", c);
    }
}
