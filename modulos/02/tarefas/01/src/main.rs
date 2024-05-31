fn main() {
  let str: &str = "Hello, world!";
  println!("{} Revertido: {}", str, reverse_string(str));
}

fn reverse_string(str: &str) -> String {
  let mut reverse: String = String::new();

  for c in str.chars().rev() {
    reverse.push(c);
  }

  return reverse;
}
