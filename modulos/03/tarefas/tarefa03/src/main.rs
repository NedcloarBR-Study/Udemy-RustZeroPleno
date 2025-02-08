extern crate rand;
use rand::Rng;
use std::io::stdin;

fn main() {
  let number = rand::thread_rng().gen_range(1, 101);
  let mut guess = String::new();

  println!("Jogo de adivinhar o número");

  loop {
    guess.clear();
    println!("Digite um número entre 1 e 100:");

    stdin()
      .read_line(&mut guess)
      .expect("Erro ao ler a entrada");

    let guess_number: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Por favor, digite um número válido.");
        continue;
      }
    };

    if guess_number == number {
      println!("O número está correto!");
      break;
    }

    if guess_number > number {
      println!("O palpite está mais alto que o número.");
    }

    if guess_number < number {
      println!("O palpite está mais baixo que o número.");
    }
  }
}
