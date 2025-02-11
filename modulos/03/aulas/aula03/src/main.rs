struct Pessoa {
  nome: String,
  idade: i32,
}

trait Voz {
  fn falar(&self);

  fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa {
  fn falar(&self) {
    println!("Olá, meu nome é: {}", self.nome);
  }

  fn tem_voz(&self) -> bool {
    if (self.idade > 1) {
      return true;
    }
    return false;
  }
}

fn main() {
  let pessoa = Pessoa {
    nome: String::from("Miguel"),
    idade: 20,
  };

  println!("Pode {} falar? {}", pessoa.nome, pessoa.tem_voz());
  if pessoa.tem_voz() {
    pessoa.falar();
  }
}
