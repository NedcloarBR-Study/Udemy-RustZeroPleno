enum Operation {
  Addition(i32, i32),
  Subtraction(i32, i32),
  Multiplication(i32, i32),
  Division(i32, i32),
}

fn calculate(op: Operation) -> Result<i32, &'static str> {
  match op {
    Operation::Addition(a, b) => Ok(a + b),
    Operation::Subtraction(a, b) => Ok(a - b),
    Operation::Multiplication(a, b) => Ok(a * b),
    Operation::Division(a, b) => {
      if b == 0 {
        Err("Cannot divide by zero.")
      } else {
        Ok(a / b)
      }
    }
  }
}

fn main() {
  let ops = vec![
    Operation::Addition(5, 3),
    Operation::Subtraction(10, 4),
    Operation::Multiplication(6, 7),
    Operation::Division(20, 5),
    Operation::Division(10, 0),
  ];

  for op in ops {
    match calculate(op) {
      Ok(result) => println!("Result: {}", result),
      Err(err) => println!("Error: {}", err),
    }
  }
}
