fn main(){
    let s :String= String::from("String");
    let ss :&str = "String";

    assert_eq!(s, ss.to_string());
    assert_eq!(ss, s.as_str());
}
