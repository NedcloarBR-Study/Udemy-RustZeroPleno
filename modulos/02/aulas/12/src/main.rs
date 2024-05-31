fn main() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{}", numbers[0]);
  for i in 0..numbers.len() {
    println!("{}", numbers[i]);
  }
  for n in numbers.iter() {
    println!("{}", n);
  }
}
