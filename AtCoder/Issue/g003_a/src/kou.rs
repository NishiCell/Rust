use std::io::*;
use std::str::*;
use std::process;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let s: String = read();
    let s: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut v = vec![0; 4];
    for i in 0..s.len() {
        if (s[i] == 'N') {
            v[0] = 1;
        }
        if (s[i] == 'W') {
            v[1] = 1;
        }
        if (s[i] == 'S') {
            v[2] = 1;
        }
        if (s[i] == 'E') {
            v[3] = 1;
        }
    }

    let ans: bool = (((v[0] ^ v[2]) | (v[1] ^ v[3])) & 1) == 0;
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock(); //what is lock() ?
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().unwrap_or_else(|_| {
        panic!(
            "Faind to tarse{
    }",
            s
        )
    })
}
