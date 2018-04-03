use std::io::*;

fn read() -> usize {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<usize>().ok().unwrap()
}
fn read_vec() -> Vec<char> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().chars().collect::<Vec<char>>()
}
fn main() {
    let h = read();
    let w = read();
    let mut vec = Vec::new();
    for c in 0..h {
        let mut vec_gyo = read_vec();
        vec_gyo.push('.');
        vec.push(vec_gyo)
    }
    vec.push(vec!['.'; w]);
    let mut c: usize = 0;
    let mut d: usize = 0;

    while c == h && d == w {
        let gyou = vec[c + 1];
        let lestu = vec[c][d + 1];
        if (gyou == '#' && lestu == '#') || (gyou == '.' && lestu == '.') {
            println!("Impossible");
            break;
        }
        if gyou == '#' && lestu == '.' {
            c += 1;
        }
        if gyou == '.' && lestu == '#' {
            d += 1;
        }
    }
    println!("Possible");
}
