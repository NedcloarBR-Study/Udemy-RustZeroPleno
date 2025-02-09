mod imobiliaria;
use imobiliaria::Imobiliaria;

fn main() {
  let mut imobiliaria1 = Imobiliaria {
    nome: String::from("Imobiliária do João"),
    endereco: String::from("Rua dos Bobos, 0"),
    imoveis: Vec::new(),
  };

  imobiliaria1.novo_imovel(String::from("Rua dos Bobos, 123"), 100_000.0, 2, 1, 50.0);

  imobiliaria1.listar_imoveis();
}
