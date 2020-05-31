pub fn run() {
  println!("");
  println!("<<<<<<<<<Day 3>>>>>>>>>");
  println!("Structs, Enum");
  println!("=============");

  structure();
  enumerator();
}

fn enumerator() {
  println!("enumerator");
  println!("==========");
  enum Operator {
    Add,
    Sub,
    Mul,
    Div,
  }

  // We can implement method for num

  impl Operator {
    fn run(&self, i: f32, j: f32) -> f32 {
      match self {
        Operator::Add => i + j,
        Operator::Sub => i - j,
        Operator::Mul => i * j,
        Operator::Div => i / j,
      }
    }
  }

  let adder = Operator::Add;
  let suber = Operator::Sub;
  let muler = Operator::Mul;
  let diver = Operator::Div;

  let num1 = 3 as f32;
  let num2 = 4 as f32;

  println!("{} + {} = {}", num1, num2, adder.run(num1, num2));
  println!("{} + {} = {}", num1, num2, suber.run(num1, num2));
  println!("{} + {} = {}", num1, num2, muler.run(num1, num2));
  println!("{} + {} = {}", num1, num2, diver.run(num1, num2));
}

fn structure() {
  println!("Structure");
  println!("=========");

  #[derive(Debug)]
  struct Point(i32, i32);

  #[derive(Debug)]
  struct Rect {
    top_left: Point,
    bottom_right: Point,
  }

  fn rect_area(rect: &Rect) -> i32 {
    // here we used a nested destructuring
    let Rect { top_left: Point(x1, y1), bottom_right: Point(x2,y2) } = rect;

    let width = x2 - x1;
    let height =  y2 - y1;
    width * height
  }

  let rect = Rect { top_left: Point(1,2), bottom_right: Point(3,4) };
  let area = rect_area(&rect);
  println!("Area of rect {:#?} is {:#?}.", rect, area);
}
