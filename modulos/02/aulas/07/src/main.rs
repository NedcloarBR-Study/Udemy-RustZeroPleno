fn main() {
  let mut x = 10;
  let y = &mut x;
  *y += 1;
  let z = &y;
  println!("{}", z);
}
