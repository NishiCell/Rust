fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim().parse::<usize>().ok().unwrap();

    let mut arr = [0, 0, 1, 1];
    for _i in 3..buf {
        arr[3] = arr[0] + arr[1] + arr[2];
        arr = [arr[1] % 10007, arr[2] % 10007, arr[3] % 10007, 0];
    }
    let ans = arr[2] as u32;
    println!("{}", ans);
}
