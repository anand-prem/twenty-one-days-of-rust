struct Number {
    odd: bool,
    value: i32,
    some: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn print_number(n: Number) {
    if let Number { odd: true, some: 222, value} = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value: 2, some } = n {
        println!("Even number: {}", some);
    }
}

fn print_number_match(n: Number) {
    match n {
        Number { value: 1, .. } => println!("One"),
        Number { value: 2, .. } => println!("Two"),
        Number { value, .. } => println!("{}", value),
        // if that last arm didn't exist, we would get a compile-time error
    }
}

pub fn run() {
    println!("<<<<<<<<<Half Hour>>>>>>>>>");
    println!("===========================");
    let one = Number { odd: true, value: 1, some: 222 };
    let two = Number { odd: false, value: 2, some: 333 };
    // print_number(one);
    print_number(two);
    println!("is positive? {}", one.is_strictly_positive());
    // println!("{} is positive? {}", two.value, two.is_strictly_positive());
}
