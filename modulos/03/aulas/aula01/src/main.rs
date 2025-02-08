use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut arquivo = File::open("./src/rust_wiki.txt").expect("Erro ao abrir o arquivo");
  let mut conteudo = String::new();
  arquivo
    .read_to_string(&mut conteudo)
    .expect("Erro ao ler o conteudo do arquivo");
  println!("O conteudo em arquivo é: \n\n {}", conteudo);
}
