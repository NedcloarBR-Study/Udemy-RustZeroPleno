pub struct Doenca {
  pub nome: String,
  pub sintomas: Vec<String>,
  pub causa: String,
  pub tratamento: String,
}

impl Doenca {
  pub fn new(nome: String, sintomas: Vec<String>, causa: String, tratamento: String) -> Doenca {
    return Doenca {
      nome,
      sintomas,
      causa,
      tratamento,
    };
  }

  pub fn get_nome(&self) -> String {
    return self.nome.clone();
  }

  pub fn get_sintomas(&self) -> Vec<String> {
    return self.sintomas.clone();
  }

  pub fn get_causa(&self) -> String {
    return self.causa.clone();
  }

  pub fn get_tratamento(&self) -> String {
    return self.tratamento.clone();
  }

  pub fn show(&self) {
    println!("Nome: {}", self.get_nome());
    println!("Sintomas: {:?}", self.get_sintomas());
    println!("Causa: {}", self.get_causa());
    println!("Tratamento: {}", self.get_tratamento());
  }
}
