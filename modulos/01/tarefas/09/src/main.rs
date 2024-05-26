fn main() {
  let str1 = "abc";
  let str2 = "bca";
  if eh_permutacao(str1, str2) {
    println!("As strings são permutações uma da outra");
  } else {
    println!("As strings não são permutações uma da outra");
  }
}

fn eh_permutacao(str1: &str, str2: &str) -> bool {
  if str1.len() != str2.len() {
    return false;
  }

  let mut char_count = [0; 128];

  for &c in str1.as_bytes() {
    char_count[c as usize] += 1;
  }

  for &c in str2.as_bytes() {
    char_count[c as usize] -= 1;
    if char_count[c as usize] < 0 {
      return false;
    }
  }

  return true;
}
