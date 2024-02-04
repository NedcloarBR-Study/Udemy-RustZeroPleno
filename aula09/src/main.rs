fn dobro(num: i32) -> i32 {
  return num * 2;
}

fn maximo(a: i32, b: i32) -> i32 {
  if a > b {
    return a;
  } else {
    return b;
  }
}

fn main() {
  println!("O dobro de 5 é: {}", dobro(5));
  println!("O maior entre de 5 e 4 é: {}", maximo(5, 4));
}
