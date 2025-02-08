fn main() {
  let nome = "Hello"; // slice string
  let nome2 = String::from("Hello World"); // string
  let hello = &nome2[0..5]; // slice string from string
  let world = &nome2[6..11];

  println!("{}", hello);
  println!("{}", nome);
  println!("{}", world);
}
