fn main() {
  let str = "A base do teto desaba.";
  println!("{} é um palindromo? {}", str, eh_palindromo(str));
}

fn eh_palindromo(str: &str) -> bool {
  let clean_str = str
    .to_lowercase()
    .replace(|c: char| !c.is_alphanumeric(), "");
  let len = clean_str.len();

  for i in 0..len / 2 {
    if clean_str.chars().nth(i) != clean_str.chars().nth(len - i - 1) {
      return false;
    }
  }

  return true;
}
