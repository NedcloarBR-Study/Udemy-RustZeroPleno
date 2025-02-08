use std::io::stdin;

fn main() {
  let mut user_message = String::new();
  println!("Digite uma mensagem: ");
  match stdin().read_line(&mut user_message) {
    Ok(_) => println!("Sucesso. Você digitou: {}", user_message.to_uppercase()),
    Err(error) => println!("Erro: {}", error),
  }
}
