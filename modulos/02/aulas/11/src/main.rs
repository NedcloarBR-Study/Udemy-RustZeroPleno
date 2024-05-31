struct User(String, String, bool, String);

fn main() {
  let mut pessoa: User = User(
    String::from("Miguel"),
    String::from("nedcloar1@hotmail.com"),
    true,
    String::from("Male"),
  );

  println!(
    "Nome: {}, email: {}, ativo: {}, Gênero: {}",
    pessoa.0, pessoa.1, pessoa.2, pessoa.3
  )

  pessoa.0 = String::from("Miguel Uhlein")
}
