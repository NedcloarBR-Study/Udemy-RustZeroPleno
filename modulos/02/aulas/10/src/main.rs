struct User {
  username: String,
  email: String,
  active: bool,
  gender: String,
}

fn user(user: &User) {
  println!("Nome do usuário: {}", user.username)
}

fn main() {
  let mut pessoa: User = User {
    username: String::from("Miguel"),
    email: String::from("nedcloar1@hotmail.com"),
    active: true,
    gender: String::from("Male"),
  };
  pessoa.active = false;
  user(&pessoa);
  user(&pessoa);
}
