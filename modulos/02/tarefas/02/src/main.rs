fn main() {
  let str1: &str = "anagrama";
  let str2: &str = "nagarama";
  println!(
    "{} é um anagrama com {}, {}",
    str1,
    str2,
    is_anagram(str1, str2)
  );
}

fn is_anagram(str1: &str, str2: &str) -> bool {
  let mut chars1: Vec<char> = str1.to_lowercase().chars().collect();
  let mut chars2: Vec<char> = str2.to_lowercase().chars().collect();

  chars1.sort();
  chars2.sort();

  return chars1 == chars2;
}
