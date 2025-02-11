mod viagens;

fn main() {
  let mut list_passageiros: Vec<viagens::Passageiro> = Vec::new();
  let mut list_voos: Vec<viagens::Voo> = Vec::new();

  viagens::add_passageiro(
    &mut list_passageiros,
    String::from("João da Silva"),
    String::from("123456"),
    25,
  );

  viagens::add_passageiro(
    &mut list_passageiros,
    String::from("Maria Oliveira"),
    String::from("654321"),
    30,
  );

  viagens::add_voo(
    &mut list_voos,
    String::from("V001"),
    String::from("São Paulo"),
    String::from("Rio de Janeiro"),
    String::from("01/01/2022"),
    String::from("08:00"),
  );

  viagens::add_voo(
    &mut list_voos,
    String::from("V002"),
    String::from("Rio de Janeiro"),
    String::from("São Paulo"),
    String::from("02/01/2022"),
    String::from("09:00"),
  );

  println!("Passageiros:");
  viagens::show_passageiros(&list_passageiros);

  println!("Voos:");
  viagens::show_voos(&list_voos);
}
