mod metodo;

fn main() {
  let usuario = metodo::Pessoa {
    nome: String::from("Miguel"),
    sobrenome: String::from("Alexandre Uhlein"),
  };

  usuario.qual_nome();
  usuario.nome_completo();
}
