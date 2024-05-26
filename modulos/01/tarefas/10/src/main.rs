fn main() {
  let str1 = "pale";
  let str2 = "pale";

  println!(
    "As strings estão a um passo de distância: {}",
    in_one_feet(str1, str2)
  )
}

fn in_one_feet(str1: &str, str2: &str) -> bool {
  let len1: i32 = str1.len() as i32;
  let len2: i32 = str2.len() as i32;

  if (len1 - len2).abs() > 1 {
    return false;
  }

  let mut edits: i32 = 0;
  let mut i: i32 = 0;
  let mut j: i32 = 0;

  while i < len1 && j < len2 {
    if str1.chars().nth(i as usize) != str2.chars().nth(j as usize) {
      edits += 1;
    }

    if len1 > len2 {
      i += 1;
    } else if len1 < len2 {
      j += 1;
    } else {
      i += 1;
      j += 1;
    }
  }

  if edits > 1 {
    return false;
  }

  return edits <= 1;
}
