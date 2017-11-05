fn main() {
  let long_lived_binding = 1;
  //This binding lives in the main function

  
  //This is a block, and has a smaller scope than the main mainfunction
  {
    let short_libed_binding = 2;
    //This binding only exists in this block

    println!("inner shert: {}", short_libed_binding);

    let long_lived_binding = 5_f32;
    //This binding *shadows* the outer one

    println!("inner long: {}", long_lived_binding);
  }
  //End of the block
  
  // println!("outer short: {}", short_libed_binding);
  //Error! `short_lived_binding` doesn`t exist in this scope
  
  println!("outer long: {}", long_lived_binding);

  let long_lived_binding = 'a';
  //This binding also *shadows* the previous binding

  println!("outer long: {}", long_lived_binding);

}