fn main() {
    let c: String = read();
    let n: f32 = read().parse::<f32>().unwrap();
    let c1: f32 = c.chars().filter(|&c| c == 'A').count() as f32;
    let c2: f32 = c.chars().filter(|&c| c == 'B').count() as f32;
    let c3: f32 = c.chars().filter(|&c| c == 'C').count() as f32;
    let c4: f32 = c.chars().filter(|&c| c == 'D').count() as f32;

    let num: f32 = (c1 * 4.0 + c2 * 3.0 + c3 * 2.0 + c4 * 1.0) / n;
    println!("{}", num);
}

fn read() -> String {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    n
}
