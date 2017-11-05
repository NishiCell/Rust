#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

struct Nil;
// A unit struct

struct Pair(i32, f32);
// A tuple struct

struct Point {
  x: f32,
  y: f32,
}
// A struct with two fields

#[allow(dead_code)]
struct Rectangle {
  p1: Point,
  p2: Point,
}
// Struct can be reused as fields of another struct

fn main() {
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age};
  //Create struct with field init shorthand

  println!("{:?}" , peter);
  // Print debug struct

  let point: Point = Point{ x: 0.3, y: 0.4};
  // Onstantiate a `Point`

  println!("point coordinates: ({}, {})" ,point.x, point.y);
  // Access the fieldsof the point
  
  let Point { x: my_x, y: my_y } = point;
  // Destructure the point using a `let` binding

  let _rectangle = Rectangle {
    p1: Point { x:my_y, y: my_x },
    p2: point,
  };

  let _nil = Nil;
  // Instantiate a unit strudt

  let pair = Pair(1, 0.1);
  // Instntiate a tuple struct

let Pair(integer, decimal) = pair; 

  println!("pair contains {:?} and {:?}", integer, decimal);
}