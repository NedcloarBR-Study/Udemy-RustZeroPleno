use std;

fn main() {
  println!("Digite uma sequencia de números reais: ");

  let mut input: String = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Erro ao ler a entrada");

  let numbers: Vec<f64> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse::<f64>().expect("Por favor, insira números reais"))
    .collect();

  let mut sum_pares: f64 = 0.0;
  let mut sum_impar: f64 = 0.0;
  let mut sum: f64 = 0.0;

  for num in &numbers {
    if num % 2.0 == 0.0 {
      sum_pares += num;
    } else {
      sum_impar += num;
    }
    sum += num;
  }

  println!("A soma dos números pares é: {}", sum_pares);
  println!("A soma dos números impares é: {}", sum_impar);
  println!("A soma dos números é: {}", sum);
}
