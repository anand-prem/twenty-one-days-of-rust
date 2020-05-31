use std::mem;

pub fn run() {
  println!("");
  println!("<<<<<<<<<Day 2>>>>>>>>>");
  println!("Tuple, Array");
  println!("============");

  tuple_array_slice();
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
  (a + b, a-b)
}

fn stat_slice(slice: &[i32]) {
  println!("Slice has length {}", slice.len())
}

fn tuple_array_slice() {
  // unpacking to assign
  let (a, b) = (5,3);
  // get the returned tuple;
  let (a, b) = calculate(a, b);
  println!("a is {} and b is {}", a, b);

  // array
  let a:[i32; 5] = [1, 2, 3, 4, 5];
  for num in a.iter() {
    println!("num is {}", num);
  }
  println!("First number is {}", a[0]);
  println!("Size of A is {} bytes", mem::size_of_val(&a));

  //slice
  stat_slice(&a);
  stat_slice(&a[0..4]);
}
