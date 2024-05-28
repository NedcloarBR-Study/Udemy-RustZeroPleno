const PI: f32 = 3.14159;

fn main() {
  println!("{}", circumference(2.32));
}

fn circumference(r: f32) -> f32 {
  return 2f32 * r * PI;
}
