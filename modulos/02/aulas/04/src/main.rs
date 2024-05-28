fn main() {
  let a = 10;

  {
    let b = 15;
    println!("Sum {} + {} = {}", a, b, a + b);
  }
}
