fn count(num: i32) {
  for i in 1..num + 1 {
    println!("Contando: {}", i);
  }
}

fn count_down(num: i32) {
  let mut input: i32 = num;
  while input > 0 {
    println!("Contando: {}", input);
    input = input - 1;
  }
}

fn main() {
  count(10);
  count_down(10);
}
