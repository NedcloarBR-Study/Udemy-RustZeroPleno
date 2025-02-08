fn main() {
  {
    let minha_string = String::from("Oi meu nome é Miguel");
    println!("{}", minha_string);
    println!("{}", minha_string.replace("Miguel", "Emanuel"));
  }

  {
    let minha_string = String::from("Fui hoje\nao mercado\ncomprar arroz");
    for i in minha_string.lines() {
      println!("( {} )", i);
    }
  }

  {
    let minha_string = String::from("Minha+mãe+é+muito+feliz");
    let token: Vec<&str> = minha_string.split("+").collect();
    println!("{:?}", token);
    println!("{}", token[1]);
  }

  {
    let minha_string = String::from("                 Meu nome é       Miguel      ");
    println!("{}", minha_string);
    println!("{}", minha_string.trim());
  }

  {
    let minha_string = String::from("Deixe uma avaliação de 5 estrelas");
    // println!("{}", minha_string[6])
    match minha_string.chars().nth(6) {
      Some(c) => println!("Sucesso. O caractere da 6 posição é: {}", c),
      None => println!("Erro. Não existe caractere na 6 posição"),
    }
  }
}
