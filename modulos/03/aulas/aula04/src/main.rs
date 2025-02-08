fn main() {
  let numero = 2;

  match numero {
    1 => println!("O numero é 1"),
    2 | 3 => println!("O numero é 2 ou 3"),
    4..10 => println!("O numero esta entre 4 e 10"),
    _ => println!("Eu não sei que numero é esse"),
  }

  let nome = "Miguel";

  match nome {
    "Miguel" => println!("Miguel é programador"),
    "Emanuel" => println!("Emanuel é estudante"),
    _ => println!("Eu não sei sua profissão"),
  }
}
