fn convet_to_pig_latin(palavra: &String) -> String {
  let vogais = ["a", "e", "i", "o", "u"];
  let (primeira_letra, resto_palavra) = palavra.split_at(1);
  let primeira_letra_vogal = vogais.contains(&primeira_letra);
  if primeira_letra_vogal {
    return format!("{palavra}-hay");
  }

  return format!("{resto_palavra}-{primeira_letra}ay");
}

fn main() {
  let palavra = String::from("time");
  let pig_latin = convet_to_pig_latin(&palavra);
  println!("{palavra} em pig-latin: {pig_latin}");
}
