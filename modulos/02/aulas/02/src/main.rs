enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
enum Gender {
  Male,
  Female,
}

#[derive(Debug)]
enum CarType {
  Fiat,
  Ford,
  Renault,
}

enum Payment {
  Money(f32),
  CreditCard(bool, f32),
  DebitCard(bool, f32),
}

fn main() {
  let player: Direction = Direction::Right;
  let player_male: Gender = Gender::Male;
  let player_female: Gender = Gender::Female;
  match player {
    Direction::Up => println!("O jogador foi pra cima"),
    Direction::Down => println!("O jogador foi para baixo"),
    Direction::Left => println!("O jogador foi para esquerda"),
    Direction::Right => println!("O jogador foi para direita"),
  }

  println!("{:?}", player_male);
  println!("{:?}", player_female);

  car_nationality(CarType::Fiat);
  car_nationality(CarType::Ford);
  car_nationality(CarType::Renault);

  let payment: Payment = Payment::CreditCard(false, 100f32);

  match payment {
    Payment::Money(f) => println!("Dinheiro, {}", f),
    Payment::CreditCard(true, x) => println!("Crédito, {}", x),
    Payment::CreditCard(false, x) => println!("Crédito Negado, {}", x),
    // Payment::DebitCard => println!("Débito"),
    _ => {}
  }
}

fn car_nationality(car: CarType) {
  match car {
    CarType::Fiat => println!("Carro italiano"),
    CarType::Ford => println!("Carro americano"),
    CarType::Renault => println!("Carro francês"),
  }
}
