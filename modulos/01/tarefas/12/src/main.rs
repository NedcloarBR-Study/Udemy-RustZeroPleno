fn main() {
  println!("Digite um número inteiro para checar se ele é um palíndromo: ");
  let mut input: String = String::new();
  std::io::stdin()
    .read_line(&mut input)
    .expect("Erro ao ler input");

  let num = input.parse::<i32>().expect("Digite um número inteiro");

  println!("O número {} é um palíndromo? {}", num, check_num(num));
}

fn check_num(x: i32) -> bool {
  if x < 0 || (x % 10 == 0 && x != 0) {
    return false;
  }

  let mut original: i32 = x;
  let mut inverse: i32 = 0;

  while original != 0 {
    let digit: i32 = original % 10;
    inverse = inverse * 10 + digit;
    original /= 10;
  }

  return x == inverse;
}
