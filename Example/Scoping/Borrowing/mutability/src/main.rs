fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("This int is: {}", borrowed_i32);
}

fn main() {
  let boxed_i32 = Box::new(5_i32);
  let stacked_i32 = 6_i32;
  // Creat a boxed i32, and a stacked i32

  borrow_i32(&boxed_i32);
  borrow_i32(&stacked_i32);
  // Borrow the contents of the box. Ownership is not taken,
  // so the contents con be borrowed again.

  {
    let _ref_to_i32: &i32 = &boxed_i32;
    // Take a reference to the data contained

    //eat_box_i32(boxed_i32);
    // Error!
    //Cant destroy `boxed_i32` while the inner value is borrowed.

    // `_ref_to_i32` goes out of scope and is no longe borrowed.
  }

  eat_box_i32(boxed_i32);
  // `boxed_i32` can noww gice up ownership to `eat_box` and be destroyed
}

