fn main() {
  let mut a: i32 = 15;
  let mut b: i32 = 40;
  while b != 0 {
    let temp: i32 = b;
    b = a % b;
    a = temp;
  }
  println!("O maior divisor comum entre 15 e 40 é: {}", a);
}
