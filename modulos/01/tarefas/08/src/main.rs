fn main() {
  let input = "Hello, World!";
  if check_unique_chars(input) {
    println!("Não contem caracteres repetidos");
  } else {
    println!("Contem caracteres repetidos");
  }
}

fn check_unique_chars(input: &str) -> bool {
  let mut char_set = [false; 128];
  for &c in input.as_bytes() {
    let index = c as usize;
    if char_set[index] {
      return false;
    }
    char_set[index] = true;
  }

  return true;
}
