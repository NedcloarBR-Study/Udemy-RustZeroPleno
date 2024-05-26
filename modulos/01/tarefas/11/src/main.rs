fn main() {
  let str = "aabccccaaa";
  let compress = compress_string(str);
  println!("Original: {}", str);
  println!("Compressed: {}", compress);
}

fn compress_string(s: &str) -> String {
  let mut compressed: String = String::new();
  let mut count: i32 = 0;

  for (i, c) in s.chars().enumerate() {
    count += 1;

    if i + 1 >= s.len() || c != s.chars().nth(i + 1).unwrap() {
      compressed.push(c);
      compressed.push_str(&count.to_string());
      count = 0;
    }
  }

  if compressed.len() >= s.len() {
    return s.to_string();
  } else {
    return compressed;
  }
}
