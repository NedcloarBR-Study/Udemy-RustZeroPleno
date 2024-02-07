use std::io;

fn main() {
  println!("Digite uma sequência de números reais: ");

  let mut input: String = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Erro ao ler input");

  let numbers: Vec<f64> = input
    .trim()
    .split_whitespace()
    .map(|x: &str| x.parse::<f64>().expect("Insira apenas números reais"))
    .collect();

  let mut total: f64 = 0.0;
  for i in &numbers {
    if i % 2.0 == 0.0 {
      total = total + i;
    }
  }
  println!("A soma dos números pares é: {}", total);
}
