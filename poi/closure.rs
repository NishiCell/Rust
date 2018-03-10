fn plus(x :i32) -> i32 {
    x + 1
}

fn main(){
    let plus_one_v2 = |y: i32| -> i32 { y + 1 };
    let plus_one_v3 = |z: i32|          z + 1  ;

    assert_eq!(2, plus(1));
    assert_eq!(2, plus_one_v2(1));
    assert_eq!(2, plus_one_v3(1));
}
