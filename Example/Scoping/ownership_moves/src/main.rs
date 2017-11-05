fn destroy_box(c: Box<i32>) {
  println!("Destroying a box that contains {}", c);
  //`c is destroyde and the memory freed`
}

fn main() {
  let x = 5u32;
  // _stack_ allocated interger
  // Isnt it "let :u32 = 5" ??

  let y = x;
  // *Copy* into `y` - no resources are moved

  println!("x is {}, and y is {}", x, y);
  // Both values can be independently used

  let a = Box::new(5i32);
  // `a` is a pointer to a _heap_ allocated integer

  println!("a contains: {}", a);

  let b = a;
  //*Move* `a` into `b`
  //the pointer address of `a` copied (not the data) into `b`
  //Both are now pointers to the same heap allocated data,
  //but `b` now owns it.

  // println!("a contains: {}", a);
  //Error! `a` can no longer access the data,
  //because ti no longer owns the heap memory

  // This function takes ownership of the heap allocated memory from `b`
  
  destroy_box(b);
  // since the heap memory has been freed at this poing, 
  // this action would rsult in dereferencing freed memory, 
  // but it`s forbidden by the vompiler

  //println!("b contains: {}", b);
  // Error! Same reason as the previous error

}
