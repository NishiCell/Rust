fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let x: f32 = input.trim().parse::<f32>().unwrap() / 1000.0;
    let vv: f32 = 0.0;
    if x < 0.1 {
        vv = 0.0;
    } else if x >= 0.1 && x < 5.0 {
        vv = x * 10.0;
    } else if x >= 6.0 && x <= 30.0 {
        vv = x + 50.0;
    } else if x >= 35.0 && x <= 70.0 {
        vv = (x - 30.0) / 5.0 + 80.0;
    } else if x >= 70.0 {
        vv = 89.0;
    }

    let vv2 = vv as u32;
    println!("{    }", zero(vv2).to_string());
}

fn zero(val: u32) {
    if val < 10 {
        let minten: String = val.to_string();
        let zero: String = String::from("0");
        zero //.push_str(&minten)
    }
}
