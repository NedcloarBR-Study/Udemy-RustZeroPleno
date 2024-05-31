fn main() {
  let mut minha_string: String = String::from("Olá meu nome é Miguel");
  println!("tamanho da string: {}", minha_string.len());
  println!("A string está vazia? {}", minha_string.is_empty());
  for token in minha_string.split_whitespace() {
    println!("{}", token)
  }

  println!(
    "O nome Miguel está na String? {}",
    minha_string.contains("Miguel")
  );
  minha_string.push_str(" Bem-vindo a aula.");
  println!("{}", minha_string);
}
