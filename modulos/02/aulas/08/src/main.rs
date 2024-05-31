struct User {
  username: String,
  email: String,
  active: bool,
  gender: String,
}

fn main() {
  let mut pessoa: User = User {
    username: String::from("Miguel"),
    email: String::from("nedcloar1@hotmail.com"),
    active: true,
    gender: String::from("Male"),
  };
  println!(
    "O nome do usuário é: {}\nemail: {}, gênero: {}",
    pessoa.username, pessoa.email, pessoa.gender
  );
  pessoa.active = false;
}
