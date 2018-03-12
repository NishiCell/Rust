use std::io::*;
use std::collections::VecDeque;
fn read_line() -> u64 {
    let mut s = String::new();
    let buf = stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}

fn read_vec() -> VecDeque<char> {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace());
    let mut vec = VecDeque::new();
    for c in s {
        vec.push_back(c);
    }
    vec
}

fn main() {
    let n = read_line();
    let mut s = read_vec();
    let mut t = read_vec();
    let mut ans = s.len() + t.len();
    let mut son_zai = true;
    while son_zai {
        son_zai = false;
        if s.pop_back() == t.pop_front() {
            ans -= 1;
            son_zai = true;
        }
    }

    println!("{}", ans);
}
