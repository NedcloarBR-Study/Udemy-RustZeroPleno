pub struct Imobiliaria {
  pub nome: String,
  pub endereco: String,
  pub imoveis: Vec<Imovel>,
}

pub struct Imovel {
  pub endereco: String,
  pub preco: f64,
  pub num_quartos: i32,
  pub num_banheiros: i32,
  pub metragem: f64,
}

impl Imobiliaria {
  pub fn novo_imovel(
    &mut self,
    endereco: String,
    preco: f64,
    num_quartos: i32,
    num_banheiros: i32,
    metragem: f64,
  ) {
    let imovel = Imovel {
      endereco,
      preco,
      num_quartos,
      num_banheiros,
      metragem,
    };
    self.imoveis.push(imovel);
  }

  pub fn listar_imoveis(&self) {
    for imovel in self.imoveis.iter() {
      println!("Imóvel: {}", imovel.endereco);
      println!("Preço: R$ {}", imovel.preco);
      println!("Quartos: {}", imovel.num_quartos);
      println!("Banheiros: {}", imovel.num_banheiros);
      println!("Metragem: {} m²", imovel.metragem);
      println!();
    }
  }
}
