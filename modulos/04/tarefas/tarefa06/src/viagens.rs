pub struct Passageiro {
  pub nome: String,
  pub numero_passaporte: String,
  pub idade: u8,
}

impl Passageiro {
  pub fn new(nome: String, numero_passaporte: String, idade: u8) -> Passageiro {
    Passageiro {
      nome,
      numero_passaporte,
      idade,
    }
  }

  pub fn show(&self) {
    println!("Nome: {}", self.nome);
    println!("Número do passaporte: {}", self.numero_passaporte);
    println!("Idade: {}", self.idade);
  }
}

pub struct Voo {
  pub codigo_voo: String,
  pub partida: String,
  pub destino: String,
  pub data_partida: String,
  pub hora_partida: String,
}

impl Voo {
  pub fn new(
    codigo_voo: String,
    partida: String,
    destino: String,
    data_partida: String,
    hora_partida: String,
  ) -> Voo {
    Voo {
      codigo_voo,
      partida,
      destino,
      data_partida,
      hora_partida,
    }
  }

  pub fn show(&self) {
    println!("Código do voo: {}", self.codigo_voo);
    println!("Partida: {}", self.partida);
    println!("Destino: {}", self.destino);
    println!("Data da partida: {}", self.data_partida);
    println!("Hora da partida: {}", self.hora_partida);
  }
}

pub fn add_passageiro(
  list: &mut Vec<Passageiro>,
  nome: String,
  numero_passaporte: String,
  idade: u8,
) {
  let passageiro = Passageiro::new(nome.clone(), numero_passaporte, idade);
  list.push(passageiro);
  println!("Passageiro adicionado: {}", nome);
}

pub fn add_voo(
  list: &mut Vec<Voo>,
  codigo_voo: String,
  partida: String,
  destino: String,
  data_partida: String,
  hora_partida: String,
) {
  let voo = Voo::new(
    codigo_voo.clone(),
    partida,
    destino,
    data_partida,
    hora_partida,
  );
  list.push(voo);
  println!("Voo adicionado: {}", codigo_voo);
}

pub fn show_passageiros(list: &Vec<Passageiro>) {
  for passageiro in list {
    passageiro.show();
    println!();
  }
}

pub fn show_voos(list: &Vec<Voo>) {
  for voo in list {
    voo.show();
    println!();
  }
}
