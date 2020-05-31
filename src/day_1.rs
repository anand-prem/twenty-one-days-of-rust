use std::fmt;

pub fn run() {
  println!("");
  println!("<<<<<<<<<Day 1>>>>>>>>>");
  println!("Print");
  println!("=====");

  print_simple();
  print_self_defined_structure();
  print_with_display_trait();
}

fn print_simple() {
  println!("==========Print Simple=========");

  println!("hello world");
  println!("{} and {}", "Alice", "Bob");
  println!("Escaped curly braces {{}}");
  println!("Escaped double curly braces {{{{}}}}");
  println!("hello {name1} {name2} {name3}", name2="Bo", name3="Ho", name1="Ko");
  println!("{} of {:b} people know binary", 1, 2);
  println!("{} of {:o} people know oct", 1, 8);
  println!("{} of {:x} people know hex", 1, 16);
}

fn print_self_defined_structure() {
  println!("==========Print Self Defined Structure=========");

  #[derive(Debug)]
  struct Person<'a> {
    name: &'a str,
    age: u8
  }

  let name = "Alice";
  let age = 24;
  let alice = Person {name, age};

  println!("Person is {:#?}", alice);
}

fn print_with_display_trait() {
  println!("==========Print Structure With Defined Structure=========");

  struct Point {
    x: i32,
    y: i32,
  }

  impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
      write!(f, "Point [{}, {}]", self.x, self.y)
    }
  }

  let point = Point {x: 3, y: 5};
  println!("Point is {}", point);
}
