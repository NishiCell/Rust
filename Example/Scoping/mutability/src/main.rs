fn main() {
  let immut_box = Box::new(5u32);

  println!("immutable_box conteins{}", immut_box);

  // *immut_box = 4;
  // Mutability error

  let mut mut_box = immut_box;
  // *Muve* the box, changing the ownership (and mutability)

  println!("mutable_box contains {}", mut_box);

  *mut_box = 4;

  println!("mutable_box now contains {}", mut_box);
}
